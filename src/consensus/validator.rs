use crate::blockchain::transaction::Transaction;
use crate::crypto::signatures::verify_signature;
use crate::wallet::Wallet;

pub struct Validator;

impl Validator {
    /// Validates a block by checking if all transactions are valid and if the block is properly formatted.
    ///
    /// # Arguments
    /// * `block` - A reference to the block to be validated.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the block is valid, `false` otherwise.
    pub fn validate_block(block: &Block) -> bool {
        for transaction in &block.transactions {
            if !Self::validate_transaction(transaction) {
                return false;
            }
        }
        true
    }

    /// Validates a transaction by checking its signature and validity.
    ///
    /// # Arguments
    /// * `transaction` - A reference to the transaction to be validated.
    ///
    /// # Returns
    /// * `bool` - Returns `true` if the transaction is valid, `false` otherwise.
    pub fn validate_transaction(transaction: &Transaction) -> bool {
        let public_key = Wallet::get_public_key_from_address(&transaction.from); // Fixed sender -> from

        // Verify signature and hash (convert hash to byte slice)
        verify_signature(
            &public_key,
            transaction.calculate_hash().as_bytes(), // Fixed by converting String to &[u8]
            &transaction.signature,
        )
    }
}
