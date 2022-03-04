script {
    use AptosFramework::AptosSystem;
    fun main(aptos_root: signer) {
        {{#each addresses}}
        AptosSystem::remove_validator(&aptos_root, @0x{{this}});
        {{/each}}
    }
}
