use crate::prover::{EthAddress, RebaseEvent, EthEventParams};
use ethabi::{ParamType, Token};
use hex::ToHex;
use near_sdk::{Balance};

/// Data that was emitted by the Ethereum Rebase event.
#[derive(Debug, Eq, PartialEq)]
pub struct EthRebaseEvent {
    pub rebaser_address: EthAddress,
    pub token: String,
    pub sender: String,
    pub epoch: u128,
    pub requested_adjustment: Balance,
} 

impl EthRebaseEvent {
    fn event_params() -> EthEventParams {
        vec![
            ("token".to_string(), ParamType::Address, true),
            ("sender".to_string(), ParamType::Address, true), 
            ("epoch".to_string(), ParamType::Uint(256), true),
            ("requested_adjustment".to_string(), ParamType::Uint(256), false),
        ]
    }

    /// Parse raw log entry data.
    pub fn from_log_entry_data(data: &[u8]) -> Self {
        let event = RebaseEvent::from_log_entry_data("Rebase", EthRebaseEvent::event_params(), data);
        let token = event.log.params[1].value.clone().to_address().unwrap().0;
        let token = (&token).encode_hex::<String>();
        let sender = event.log.params[2].value.clone().to_address().unwrap().0;
        let sender = (&sender).encode_hex::<String>();
        let epoch = event.log.params[3]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        let requested_adjustment = event.log.params[4]
            .value
            .clone()
            .to_uint()
            .unwrap()
            .as_u128();
        Self {
            rebaser_address: event.rebaser_address,
            token,
            sender,
            epoch,
            requested_adjustment,
        }
    }

    pub fn to_log_entry_data(&self) -> Vec<u8> {
        RebaseEvent::to_log_entry_data(
            "Rebase",
            EthRebaseEvent::event_params(),
            self.rebaser_address,
            vec![
                hex::decode(self.token.clone()).unwrap(),
                hex::decode(self.sender.clone()).unwrap(),
            ],
            vec![
                Token::Uint(self.epoch.into()),
                Token::Uint(self.requested_adjustment.into()),
            ],
        )
    }
}

impl std::fmt::Display for EthRebaseEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "token: {}; sender: {}; epoch: {} adjustment: {};",
            self.token, self.sender, self.epoch, self.requested_adjustment
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_data() {
        let event_data = EthRebaseEvent {
            rebaser_address: [0u8; 20],
            token: "6b175474e89094c44da98b954eedeac495271d0f".to_string(),
            sender: "00005474e89094c44da98b954eedeac495271d0f".to_string(),
            epoch: 10,
            requested_adjustment: 1000,
        };
        let data = event_data.to_log_entry_data();
        let result = EthRebaseEvent::from_log_entry_data(&data);
        assert_eq!(result, event_data);
    }
}
