use std::str::FromStr;
use solana_sdk::pubkey::Pubkey;
use solana_rpc_client::rpc_client::RpcClient;
use solana_program::vote::state::VoteState;

const MAINNET: &str = "https://api.mainnet-beta.solana.com/";
const TESTNET: &str = "https://api.testnet.solana.com";
const DEVNET: &str = "https://api.devnet.solana.com";

fn main() {
    let network = MAINNET;
    let rpc = RpcClient::new(network);

    println!("checking {}", network);
    let vote_accounts = rpc.get_vote_accounts().unwrap();
    println!("found {} current and {} delinquent vote accounts", vote_accounts.current.len(), vote_accounts.delinquent.len());

    let mut pubkeys = vec![];
    for account in vote_accounts.current {
        pubkeys.push(Pubkey::from_str(&account.vote_pubkey).unwrap());
    }
    for account in vote_accounts.delinquent {
        pubkeys.push(Pubkey::from_str(&account.vote_pubkey).unwrap());
    }

    let (mut ancient, mut old, mut current) = (0,0,0);
    for pubkey in pubkeys {
        let data = rpc.get_account_data(&pubkey).unwrap();
        match data[0] {
            0 => ancient += 1,
            1 => old += 1,
            2 => current += 1,
            _ => panic!("bad vote account {}", pubkey),
        }

        let theirs = VoteState::deserialize(&data).unwrap();
        let mut ours = VoteState::default();
        VoteState::deserialize_into(&data, &mut ours).unwrap();

        assert_eq!(theirs, ours);
    }

    println!("all parsed! got {} ancient, {} old, and {} current", ancient, old, current);
}
