#[cfg(not(feature = "imported"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    BankMsg, Binary, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult, Uint128,
};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use terra_cosmwasm::{TerraQuerier, TerraQueryWrapper};

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn execute(
    deps: DepsMut<TerraQueryWrapper>,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SendToBurnAccount {} => send_to_burn_account(deps, env),
    }
}

fn send_to_burn_account(deps: DepsMut<TerraQueryWrapper>, env: Env) -> StdResult<Response> {
    let balances: Vec<Coin> = deps.querier.query_all_balances(&env.contract.address)?;
    let amount = deduct_tax(deps, balances)?;
    Ok(Response::new().add_message(CosmosMsg::Bank(BankMsg::Send {
        to_address: "terra1sk06e3dyexuq4shw77y3dsv480xv42mq73anxu".to_string(),
        amount,
    })))
}

static DECIMAL_FRACTION: u128 = 1_000_000_000_000_000_000u128;
fn deduct_tax(deps: DepsMut<TerraQueryWrapper>, coins: Vec<Coin>) -> StdResult<Vec<Coin>> {
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate: Decimal = (terra_querier.query_tax_rate()?).rate;

    coins
        .into_iter()
        .map(|v| {
            let tax_cap: Uint128 = (terra_querier.query_tax_cap(v.denom.to_string())?).cap;

            Ok(Coin {
                amount: Uint128::from(
                    v.amount.u128()
                        - std::cmp::min(
                            v.amount.multiply_ratio(
                                DECIMAL_FRACTION,
                                (Uint128::from(DECIMAL_FRACTION)*tax_rate).u128() + DECIMAL_FRACTION,
                            ),
                            tax_cap,
                        )
                        .u128(),
                ),
                denom: v.denom,
            })
        })
        .collect()
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(Binary::default())
}
