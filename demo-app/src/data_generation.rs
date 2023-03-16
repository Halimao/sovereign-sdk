use crate::{
    runtime::Runtime,
    tx_verifier::{RawTx, Transaction},
};
use borsh::BorshSerialize;
use sov_modules_api::mocks::{MockContext, MockPublicKey, MockSignature};

pub(crate) fn simulate_da() -> Vec<RawTx> {
    let mut messages = Vec::default();
    messages.extend(CallGenerator::election_call_messages());
    messages.extend(CallGenerator::value_setter_call_messages());
    messages
}

// Test helpers
struct CallGenerator {}

impl CallGenerator {
    fn election_call_messages() -> Vec<RawTx> {
        let mut messages = Vec::default();

        let admin = MockPublicKey::try_from("admin").unwrap();

        let set_candidates_message = election::call::CallMessage::<MockContext>::SetCandidates {
            names: vec!["candidate_1".to_owned(), "candidate_2".to_owned()],
        };

        messages.push((admin.clone(), set_candidates_message));

        let voters = vec![
            MockPublicKey::try_from("voter_1").unwrap(),
            MockPublicKey::try_from("voter_2").unwrap(),
            MockPublicKey::try_from("voter_3").unwrap(),
        ];

        for voter in voters {
            let add_voter_message =
                election::call::CallMessage::<MockContext>::AddVoter(voter.clone());

            messages.push((admin.clone(), add_voter_message));

            let vote_message = election::call::CallMessage::<MockContext>::Vote(1);
            messages.push((voter, vote_message));
        }

        let freeze_message = election::call::CallMessage::<MockContext>::FreezeElection;
        messages.push((admin, freeze_message));

        messages
            .into_iter()
            .map(|(sender, m)| RawTx {
                data: Transaction::new(Runtime::<MockContext>::encode_election_call(m), sender)
                    .try_to_vec()
                    .unwrap(),
            })
            .collect()
    }

    fn value_setter_call_messages() -> Vec<RawTx> {
        let admin = MockPublicKey::try_from("admin").unwrap();
        let new_value = 99;

        let set_value_msg_1 =
            value_setter::call::CallMessage::DoSetValue(value_setter::call::SetValue { new_value });

        let new_value = 33;
        let set_value_msg_2 =
            value_setter::call::CallMessage::DoSetValue(value_setter::call::SetValue { new_value });

        vec![
            RawTx {
                data: Transaction::new(
                    Runtime::<MockContext>::encode_value_setter_call(set_value_msg_1),
                    admin.clone(),
                )
                .try_to_vec()
                .unwrap(),
            },
            RawTx {
                data: Transaction::new(
                    Runtime::<MockContext>::encode_value_setter_call(set_value_msg_2),
                    admin,
                )
                .try_to_vec()
                .unwrap(),
            },
        ]
    }
}

pub(crate) struct QueryGenerator {}

impl QueryGenerator {
    pub(crate) fn generate_query_election_message() -> Vec<u8> {
        let query_message = election::query::QueryMessage::GetResult;
        Runtime::<MockContext>::encode_election_query(query_message)
    }

    pub(crate) fn generate_query_value_setter_message() -> Vec<u8> {
        let query_message = value_setter::query::QueryMessage::GetValue;
        Runtime::<MockContext>::encode_value_setter_query(query_message)
    }
}

impl Transaction<MockContext> {
    pub fn new(msg: Vec<u8>, pub_key: MockPublicKey) -> Self {
        Self {
            signature: MockSignature {
                msg_sig: Vec::default(),
            },
            runtime_msg: msg,
            pub_key,
            nonce: 0,
        }
    }
}