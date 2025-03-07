use super::*;

#[derive(Serialize, Deserialize)]
pub struct CardinalUtxo {
  pub output: OutPoint,
  pub amount: u64,
}

pub(crate) fn run(wallet: Wallet) -> SubcommandResult {
  let unspent_outputs = wallet.utxos();

  let inscribed_utxos = wallet
    .inscriptions()
    .keys()
    .map(|satpoint| satpoint.outpoint)
    .collect::<BTreeSet<OutPoint>>();

  let runic_utxos = wallet.get_runic_outputs()?.unwrap_or_default();

  let cardinal_utxos = unspent_outputs
    .iter()
    .filter_map(|(output, txout)| {
      if inscribed_utxos.contains(output) || runic_utxos.contains(output) {
        None
      } else {
        Some(CardinalUtxo {
          output: *output,
          amount: txout.value.to_sat(),
        })
      }
    })
    .collect::<Vec<CardinalUtxo>>();

  Ok(Some(Box::new(cardinal_utxos)))
}
