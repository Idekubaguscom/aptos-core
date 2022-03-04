script {
    use AptosFramework::ParallelExecutionConfig;
    fun main(aptos_root: signer, _execute_as: signer, payload: vector<u8>) {
        ParallelExecutionConfig::enable_parallel_execution_with_config(&aptos_root, payload);
    }
}
