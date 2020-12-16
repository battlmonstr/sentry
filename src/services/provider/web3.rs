use super::*;

fn web3_block_to_header<TX>(block: ::web3::types::Block<TX>) -> Option<Header> {
    Some(Header {
        parent_hash: block.parent_hash,
        ommers_hash: block.uncles_hash,
        beneficiary: block.author,
        state_root: block.state_root,
        transactions_root: block.transactions_root,
        receipts_root: block.receipts_root,
        logs_bloom: block.logs_bloom?,
        difficulty: block.difficulty,
        number: block.number?.as_u64().into(),
        gas_limit: block.gas_limit,
        gas_used: block.gas_used,
        timestamp: block.timestamp.as_u64(),
        extra_data: block.extra_data.0,
        mix_hash: block.mix_hash?,
        nonce: block.nonce?,
    })
}

fn web3_header_to_header(header: ::web3::types::BlockHeader) -> Option<Header> {
    Some(Header {
        parent_hash: header.parent_hash,
        ommers_hash: header.uncles_hash,
        beneficiary: header.author,
        state_root: header.state_root,
        transactions_root: header.transactions_root,
        receipts_root: header.receipts_root,
        logs_bloom: header.logs_bloom,
        difficulty: header.difficulty,
        number: header.number?.as_u64().into(),
        gas_limit: header.gas_limit,
        gas_used: header.gas_used,
        timestamp: header.timestamp.as_u64(),
        extra_data: header.extra_data.0,
        mix_hash: header.mix_hash?,
        nonce: header.nonce?,
    })
}

#[derive(Deserialize)]
struct JsonRpcResponse<T> {
    result: T,
}
#[derive(Debug)]
pub struct Web3DataProvider {
    client: ::web3::Web3<::web3::transports::Http>,
    http_client: reqwest::Client,
    addr: String,
}

impl Web3DataProvider {
    pub fn new(addr: String) -> anyhow::Result<Self> {
        let transport = ::web3::transports::Http::new(&addr)?;
        let client = ::web3::Web3::new(transport);
        let http_client = reqwest::Client::new();

        Ok(Self {
            client,
            addr,
            http_client,
        })
    }

    async fn get_transaction(&self, id: H256) -> anyhow::Result<Transaction> {
        let raw = self
            .client
            .eth()
            .transaction(id.into())
            .compat()
            .await?
            .ok_or_else(|| anyhow!("Transaction not found"))?
            .raw
            .ok_or_else(|| anyhow!("Raw transaction data absent"))?
            .0;

        Ok(rlp::decode(&*raw)?)
    }
}

#[async_trait]
impl DataProvider for Web3DataProvider {
    async fn get_status_data(&self) -> anyhow::Result<StatusData> {
        let network_id = 1;

        let best_block_number = self.client.eth().block_number().compat().await?.as_u64();
        let best_block = self
            .client
            .eth()
            .block(U64::from(best_block_number).into())
            .compat()
            .await?
            .ok_or_else(|| anyhow!("no current block?"))?;

        let total_difficulty = best_block
            .total_difficulty
            .ok_or_else(|| anyhow!("no total difficulty in best block"))?;

        let best_hash = best_block
            .hash
            .ok_or_else(|| anyhow!("no best block hash?"))?;

        let fork_data = async {
            Ok::<_, anyhow::Error>(
                self.http_client
                    .post(&self.addr)
                    .json(&json!({
                        "jsonrpc": "2.0",
                        "method": "tg_forks",
                        "params": [],
                        "id": 1,
                    }))
                    .send()
                    .await?
                    .json::<JsonRpcResponse<_>>()
                    .await?
                    .result,
            )
        }
        .compat()
        .await?;

        Ok(StatusData {
            network_id,
            total_difficulty,
            best_hash,
            fork_data,
        })
    }

    async fn resolve_block_height(&self, block: H256) -> anyhow::Result<Option<u64>> {
        Ok(self
            .client
            .eth()
            .block(block.into())
            .compat()
            .await?
            .and_then(|block| block.number)
            .map(|v| v.as_u64()))
    }

    fn get_block_headers(&self, blocks: Vec<BlockId>) -> BoxStream<anyhow::Result<Header>> {
        Box::pin(
            blocks
                .into_iter()
                .map(|block| async move {
                    let block = self
                        .client
                        .eth()
                        .block(block.into())
                        .compat()
                        .await?
                        .ok_or_else(|| anyhow!("Block not found"))?;

                    web3_block_to_header(block).ok_or_else(|| anyhow!("Pending block"))
                })
                .collect::<FuturesOrdered<_>>(),
        )
    }

    #[instrument(skip(self))]
    fn get_block_bodies(&self, blocks: Vec<H256>) -> BoxStream<anyhow::Result<BlockBody>> {
        Box::pin(
            blocks
                .iter()
                .map(|&id| {
                    async move {
                        let block = self
                            .client
                            .eth()
                            .block_with_txs(id.into())
                            .compat()
                            .await?
                            .ok_or_else(|| anyhow!("Block not found"))?;

                        trace!("Got block: {:?}", block);

                        let ommers = async {
                            Ok::<_, anyhow::Error>(
                                block
                                    .uncles
                                    .iter()
                                    .enumerate()
                                    .map(|(i, hash)| async move {
                                        trace!("fetching uncle {}/{}", i, hash);
                                        web3_header_to_header(
                                            self.client
                                                .eth()
                                                .uncle_header(id.into(), i.into())
                                                .compat()
                                                .await?
                                                .ok_or_else(|| anyhow!("Uncle not found"))?,
                                        )
                                        .ok_or_else(|| anyhow!("Pending block?"))
                                    })
                                    .collect::<FuturesOrdered<_>>()
                                    .collect::<anyhow::Result<_>>()
                                    .await
                                    .context("Failed to fetch uncles")?,
                            )
                        }
                        .await?;

                        let transactions = block
                            .transactions
                            .into_iter()
                            .map(|tx| {
                                Ok(Transaction {
                                    nonce: tx.nonce,
                                    gas_price: tx.gas_price,
                                    gas_limit: tx.gas,
                                    action: if let Some(to) = tx.to {
                                        TransactionAction::Call(to)
                                    } else {
                                        TransactionAction::Create
                                    },
                                    value: tx.value,
                                    signature: TransactionSignature::new(
                                        tx.v.ok_or_else(|| anyhow!("no v"))?.as_u64(),
                                        <[u8; 32]>::from(tx.r.ok_or_else(|| anyhow!("no r"))?)
                                            .into(),
                                        <[u8; 32]>::from(tx.s.ok_or_else(|| anyhow!("no s"))?)
                                            .into(),
                                    )
                                    .ok_or_else(|| anyhow!("invalid signature"))?,
                                    input: tx.input.0,
                                })
                            })
                            .collect::<anyhow::Result<_>>()?;

                        Ok(BlockBody {
                            transactions,
                            ommers,
                        })
                    }
                    .instrument(span!(
                        Level::TRACE,
                        "get block bodies",
                        "block={:?}",
                        id,
                    ))
                })
                .collect::<FuturesOrdered<_>>(),
        )
    }
}
