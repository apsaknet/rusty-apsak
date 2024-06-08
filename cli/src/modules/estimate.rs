use crate::imports::*;
use apsak_wallet_core::tx::PaymentDestination;

#[derive(Default, Handler)]
#[help("Estimate the fees for a transaction of a given amount")]
pub struct Estimate;

impl Estimate {
    async fn main(self: Arc<Self>, ctx: &Arc<dyn Context>, argv: Vec<String>, _cmd: &str) -> Result<()> {
        let ctx = ctx.clone().downcast_arc::<ApsakCli>()?;

        let account = ctx.wallet().account()?;

        if argv.is_empty() {
            tprintln!(ctx, "usage: estimate <amount> [<priority fee>]");
            return Ok(());
        }

        let amount_ipmos = try_parse_required_nonzero_apsak_as_ipmos_u64(argv.first())?;
        let priority_fee_ipmos = try_parse_optional_apsak_as_ipmos_i64(argv.get(1))?.unwrap_or(0);
        let abortable = Abortable::default();

        // just use any address for an estimate (change address)
        let change_address = account.change_address()?;
        let destination = PaymentDestination::PaymentOutputs(PaymentOutputs::from((change_address.clone(), amount_ipmos)));
        let estimate = account.estimate(destination, priority_fee_ipmos.into(), None, &abortable).await?;

        tprintln!(ctx, "Estimate - {estimate}");

        Ok(())
    }
}
