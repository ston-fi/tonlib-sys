extern "C" {
    /**
     * @brief Creates TransactionEmulator object
     * @param config_params_boc Base64 encoded BoC serialized Config dictionary (Hashmap 32 ^Cell)
     * @param vm_log_verbosity Verbosity level of VM log. 0 - log truncated to last 256 characters. 1 - unlimited length log.
     * 2 - for each command prints its cell hash and offset. 3 - for each command log prints all stack values.
     * @return Pointer to TransactionEmulator or nullptr in case of error
     */
    pub fn transaction_emulator_create(
        config_params_boc: *const std::os::raw::c_char,
        vm_log_verbosity: u32,
    ) -> *mut std::os::raw::c_void;

    /**
     * @brief Set unixtime for emulation
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param unixtime Unix timestamp
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_unixtime(
        tx_emulator: *const std::os::raw::c_void,
        unix_time: u32,
    ) -> bool;

    /**
     * @brief Set lt for emulation
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param lt Logical time
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_lt(
        tx_emulator: *const std::os::raw::c_void,
        lt: u64,
    ) -> bool;

    /**
     * @brief Set rand seed for emulation
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param rand_seed_hex Hex string of length 64
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_rand_seed(
        tx_emulator: *const std::os::raw::c_void,
        rand_seed_hex: *const std::os::raw::c_char,
    ) -> bool;

    /**
     * @brief Set ignore_chksig flag for emulation
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param ignore_chksig Whether emulation should always succeed on CHKSIG operation
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_ignore_chksig(
        tx_emulator: *const std::os::raw::c_void,
        ignore_chksig: bool,
    ) -> bool;

    /**
     * @brief Set ignore_chksig flag for emulation
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param ignore_chksig Whether emulation should always succeed on CHKSIG operation
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_config(
        tx_emulator: *const std::os::raw::c_void,
        config_boc: *const std::os::raw::c_char,
    ) -> bool;

    /**
     * @brief Set libs for emulation
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param libs_boc Base64 encoded BoC serialized shared libraries dictionary (HashmapE 256 ^Cell).
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_libs(
        tx_emulator: *const std::os::raw::c_void,
        libs_boc: *const std::os::raw::c_char,
    ) -> bool;

    /**
     * @brief Enable or disable TVM debug primitives
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param debug_enabled Whether debug primitives should be enabled or not
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_debug_enabled(
        tx_emulator: *const std::os::raw::c_void,
        debug_enabled: bool,
    ) -> bool;

    /**
     * @brief Set tuple of previous blocks (13th element of c7)
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param info_boc Base64 encoded BoC serialized TVM tuple (VmStackValue).
     * @return true in case of success, false in case of error
     */
    pub fn transaction_emulator_set_prev_blocks_info(
        tx_emulator: *const std::os::raw::c_void,
        info_boc: *const std::os::raw::c_char,
    ) -> bool;

    /**
     * @brief Emulate transaction
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param shard_account_boc Base64 encoded BoC serialized ShardAccount
     * @param message_boc Base64 encoded BoC serialized inbound Message (internal or external)
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description",
     *   "external_not_accepted": false,
     *   // and optional fields "vm_exit_code", "vm_log", "elapsed_time" in case external message was not accepted.
     * }
     * Or success:
     * {
     *   "success": true,
     *   "transaction": "Base64 encoded Transaction boc",
     *   "shard_account": "Base64 encoded new ShardAccount boc",
     *   "vm_log": "execute DUP...",
     *   "actions": "Base64 encoded compute phase actions boc (OutList n)",
     *   "elapsed_time": 0.02
     * }
     */
    pub fn transaction_emulator_emulate_transaction(
        tx_emulator: *const std::os::raw::c_void,
        shard_account_boc: *const std::os::raw::c_char,
        message_boc: *const std::os::raw::c_char,
    ) -> *const std::os::raw::c_char;

    /**
     * @brief Emulate transaction
     * @param transaction_emulator Pointer to TransactionEmulator object
     * @param shard_account_boc Base64 encoded BoC serialized ShardAccount
     * @param message_boc Base64 encoded BoC serialized inbound Message (internal or external)
     * @return Json object with error:
     * {
     *   "success": false,
     *   "error": "Error description",
     *   "external_not_accepted": false,
     *   // and optional fields "vm_exit_code", "vm_log", "elapsed_time" in case external message was not accepted.
     * }
     * Or success:
     * {
     *   "success": true,
     *   "transaction": "Base64 encoded Transaction boc",
     *   "shard_account": "Base64 encoded new ShardAccount boc",
     *   "vm_log": "execute DUP...",
     *   "actions": "Base64 encoded compute phase actions boc (OutList n)",
     *   "elapsed_time": 0.02
     * }
     */
    pub fn transaction_emulator_emulate_tick_tock_transaction(
        tx_emulator: *const std::os::raw::c_void,
        shard_account_boc: *const std::os::raw::c_char,
        is_tock: bool,
    ) -> *const std::os::raw::c_char;

    /**
     * @brief Destroy TransactionEmulator object
     * @param transaction_emulator Pointer to TransactionEmulator object
     */
    pub fn transaction_emulator_destroy(
        tx_emulator: *const std::os::raw::c_void,
    );
}