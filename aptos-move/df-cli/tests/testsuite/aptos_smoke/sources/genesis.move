script {
    use AptosFramework::AccountFreezing;
    use AptosFramework::ChainId;
    use AptosFramework::XUS;
    use AptosFramework::DualAttestation;
    use AptosFramework::XDX;
    use AptosFramework::Aptos;
    use AptosFramework::AptosAccount;
    use AptosFramework::AptosBlock;
    use AptosFramework::AptosConfig;
    use AptosFramework::AptosSystem;
    use AptosFramework::AptosTimestamp;
    use AptosFramework::AptosTransactionPublishingOption;
    use AptosFramework::AptosVersion;
    use AptosFramework::TransactionFee;
    use AptosFramework::AptosVMConfig;
    use Std::Vector;

    fun genesis(
        dr_account: signer,
        tc_account: signer,
    ) {
        let dr_account = &dr_account;
        let tc_account = &tc_account;
        let dummy_auth_key = x"0000000000000000000000000000000000000000000000000000000000000000";
        let dr_auth_key = copy dummy_auth_key;
        let tc_auth_key = dummy_auth_key;

        // no script allowlist + allow open publishing
        let initial_script_allow_list = Vector::empty();
        let is_open_module = true;
        let instruction_schedule = Vector::empty();
        let native_schedule = Vector::empty();
        let chain_id = 0;
        let initial_aptos_version = 1;

        AptosAccount::initialize(dr_account, x"00000000000000000000000000000000");

        ChainId::initialize(dr_account, chain_id);

        // On-chain config setup
        AptosConfig::initialize(dr_account);

        // Currency setup
        Aptos::initialize(dr_account);

        // Currency setup
        XUS::initialize(dr_account, tc_account);

        XDX::initialize(
            dr_account,
            tc_account,
        );

        AccountFreezing::initialize(dr_account);

        TransactionFee::initialize(tc_account);

        AptosSystem::initialize_validator_set(dr_account);
        AptosVersion::initialize(dr_account, initial_aptos_version);
        DualAttestation::initialize(dr_account);
        AptosBlock::initialize_block_metadata(dr_account);

        let dr_rotate_key_cap = AptosAccount::extract_key_rotation_capability(dr_account);
        AptosAccount::rotate_authentication_key(&dr_rotate_key_cap, dr_auth_key);
        AptosAccount::restore_key_rotation_capability(dr_rotate_key_cap);

        AptosTransactionPublishingOption::initialize(
            dr_account,
            initial_script_allow_list,
            is_open_module,
        );

        AptosVMConfig::initialize(
            dr_account,
            instruction_schedule,
            native_schedule,
        );

        let tc_rotate_key_cap = AptosAccount::extract_key_rotation_capability(tc_account);
        AptosAccount::rotate_authentication_key(&tc_rotate_key_cap, tc_auth_key);
        AptosAccount::restore_key_rotation_capability(tc_rotate_key_cap);
        AptosTimestamp::set_time_has_started_for_testing(dr_account);
    }

}
