script {
    use AptosFramework::AptosTransactionPublishingOption;
    fun main(aptos_root: signer) {
        AptosTransactionPublishingOption::halt_all_transactions(&aptos_root);
    }
}
