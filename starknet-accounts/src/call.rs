use starknet_core::types::FieldElement;

#[derive(Debug, Clone)]
pub struct Call {
    pub to: FieldElement,
    pub selector: FieldElement,
    pub calldata: Vec<FieldElement>,
}

impl Call {
    pub fn encode(calls: &[Call], nonce: FieldElement) -> Vec<FieldElement> {
        let mut concated_calldata: Vec<FieldElement> = vec![];
        let mut execute_calldata: Vec<FieldElement> = vec![calls.len().into()];
        for call in calls.iter() {
            execute_calldata.push(call.to); // to
            execute_calldata.push(call.selector); // selector
            execute_calldata.push(concated_calldata.len().into()); // data_offset
            execute_calldata.push(call.calldata.len().into()); // data_len

            for item in call.calldata.iter() {
                concated_calldata.push(*item);
            }
        }
        execute_calldata.push(concated_calldata.len().into()); // calldata_len
        execute_calldata.extend(concated_calldata); // calldata
        execute_calldata.push(nonce); // nonce
        execute_calldata
    }

    pub fn decode(mut felts: Vec<FieldElement>) -> (Vec<Call>, FieldElement) {
        // parse calls number
        let (calls_len, left) = felts.split_at(1);
        let calls_len = calls_len[0].to_bytes_be()[32] as usize;
        let mut call_vec = Vec::with_capacity(calls_len);
        // parse call field
        let (calls, left) = left.split_at(calls_len * 4);
        let (call_data_len, call_data) = left.split_at(1);
        assert_eq!(call_data_len[0], call_data.len().into());
        for call in calls.chunks_exact(calls_len){
            let offset = call[2].to_bytes_be()[32] as usize;
            let data_len = call[3].to_bytes_be()[32] as usize;
            call_vec.push(
                Call{
                    to: call[0],
                    selector: call[1],
                    calldata: call_data[offset..offset + data_len].to_vec()
                }
            )
        }
        let nonce = *felts.last().unwrap();
        (call_vec, nonce)
    }
}
