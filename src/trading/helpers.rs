use solana_sdk::{
    pubkey::Pubkey,
    pubkey::PubkeyError,
};

/// Token Program ID (Pubkey 类型)
/// TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
pub const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131,
    11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
]);

/// Token Program 2022 ID (Pubkey 类型)
/// TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb
pub const TOKEN_PROGRAM_2022_ID: Pubkey = Pubkey::new_from_array([
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
    28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
]);

/// Associated Token Program ID
pub const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

/// 计算关联代币账户地址
/// 
/// 这是 ATA 的标准计算方式：PDA(owner, token_program_id, mint)
pub fn get_associated_token_address(owner: &Pubkey, mint: &Pubkey) -> Pubkey {
    let associated_token_program_id: Pubkey = ASSOCIATED_TOKEN_PROGRAM_ID
        .parse()
        .expect("Invalid Associated Token Program ID");
    
    let (address, _bump) = Pubkey::find_program_address(
        &[
            owner.as_ref(),
            TOKEN_PROGRAM_ID.as_ref(),
            mint.as_ref(),
        ],
        &associated_token_program_id,
    );
    address
}

/// Pump 程序 ID
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

/// PumpAMM 程序 ID
pub const PUMP_AMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

/// 获取 Pump 程序 ID
pub fn pump_program_id() -> Pubkey {
    PUMP_PROGRAM_ID.parse().expect("Invalid Pump Program ID")
}

/// 获取 PumpAMM 程序 ID
pub fn pump_amm_program_id() -> Pubkey {
    PUMP_AMM_PROGRAM_ID.parse().expect("Invalid PumpAMM Program ID")
}

/// 派生 Global PDA
pub fn derive_global_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global"], program_id)
}

/// 派生 Bonding Curve PDA
pub fn derive_bonding_curve_pda(mint: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"bonding-curve", mint.as_ref()], program_id)
}

/// 派生 Associated Bonding Curve Token Account
pub fn derive_associated_bonding_curve(
    bonding_curve: &Pubkey,
    mint: &Pubkey,
) -> Pubkey {
    get_associated_token_address(bonding_curve, mint)
}

/// 派生 User Associated Token Account
pub fn derive_user_associated_token_account(
    user: &Pubkey,
    mint: &Pubkey,
) -> Pubkey {
    get_associated_token_address(user, mint)
}

/// 派生 Creator Vault PDA
pub fn derive_creator_vault_pda(creator: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"creator-vault", creator.as_ref()], program_id)
}

/// 派生 Fee Config PDA
/// 注意：根据 IDL，fee_config 的 seeds 包含一个固定的 fee_recipient 地址
pub fn derive_fee_config_pda(
    _fee_recipient: &Pubkey,
    fee_program: &Pubkey,
) -> Result<(Pubkey, u8), PubkeyError> {
    // 根据 IDL，fee_config 的 seeds 是：
    // [b"fee_config", fee_recipient_bytes]
    // 其中 fee_recipient_bytes 是固定的
    let fee_recipient_bytes = [
        1, 86, 224, 246, 147, 102, 90, 207, 68, 219, 21, 104, 191, 23, 91, 170, 
        81, 137, 203, 151, 245, 210, 255, 59, 101, 93, 43, 182, 253, 109, 24, 176
    ];
    Ok(Pubkey::find_program_address(
        &[b"fee_config", &fee_recipient_bytes],
        fee_program,
    ))
}

/// 派生 Event Authority PDA
pub fn derive_event_authority_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"__event_authority"], program_id)
}

/// 派生 Global Volume Accumulator PDA
pub fn derive_global_volume_accumulator_pda(
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global_volume_accumulator"], program_id)
}

/// 派生 User Volume Accumulator PDA
pub fn derive_user_volume_accumulator_pda(
    user: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"user_volume_accumulator", user.as_ref()], program_id)
}

/// 默认 Fee Recipient 地址（普通模式）
/// 62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV
pub const FEE_RECIPIENT: Pubkey = Pubkey::new_from_array([
    62, 113, 195, 14, 88, 114, 169, 81, 222, 161, 100, 39, 118, 226, 191, 13,
    131, 11, 90, 19, 153, 218, 255, 16, 132, 4, 142, 123, 216, 219, 233, 248,
]);

/// Mayhem Mode Fee Recipient 地址
/// GesfTA3X2arioaHp8bbKdjG9vJtskViWACZoYvxp4twS
pub const MAYHEM_FEE_RECIPIENT: Pubkey = Pubkey::new_from_array([
    71, 101, 115, 102, 84, 65, 51, 88, 50, 97, 114, 105, 111, 97, 72, 112,
    56, 98, 98, 75, 100, 106, 71, 57, 118, 74, 116, 115, 107, 86, 105, 87,
]);

/// 获取 Fee Recipient 地址（默认地址）
/// 注意：实际使用时应该从 Global 账户中读取
pub fn get_default_fee_recipient() -> Pubkey {
    FEE_RECIPIENT
}

/// 获取 Fee Recipient 地址（根据 mayhem mode）
pub fn get_fee_recipient(is_mayhem_mode: bool) -> Pubkey {
    if is_mayhem_mode {
        MAYHEM_FEE_RECIPIENT
    } else {
        FEE_RECIPIENT
    }
}

/// 获取 Token Program ID（根据 mayhem mode）
pub fn get_token_program_id(is_mayhem_mode: bool) -> Pubkey {
    if is_mayhem_mode {
        TOKEN_PROGRAM_2022_ID
    } else {
        TOKEN_PROGRAM_ID
    }
}

/// Fee Program ID
pub const FEE_PROGRAM_ID: &str = "pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ";

/// 获取 Fee Program ID
pub fn fee_program_id() -> Pubkey {
    FEE_PROGRAM_ID.parse().expect("Invalid Fee Program ID")
}

/// 派生 PumpAMM Global Config PDA
pub fn derive_pump_amm_global_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global_config"], program_id)
}

/// 派生 PumpAMM Pool PDA
/// 
/// # 参数
/// 
/// * `index` - Pool 索引
/// * `creator` - Pool 创建者
/// * `base_mint` - Base token mint
/// * `quote_mint` - Quote token mint (通常是 SOL/WSOL)
/// * `program_id` - PumpAMM 程序 ID
pub fn derive_pump_amm_pool_pda(
    index: u16,
    creator: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    let index_bytes = index.to_le_bytes();
    Pubkey::find_program_address(
        &[
            b"pool",
            &index_bytes,
            creator.as_ref(),
            base_mint.as_ref(),
            quote_mint.as_ref(),
        ],
        program_id,
    )
}

/// 派生 PumpAMM Coin Creator Vault Authority PDA
pub fn derive_pump_amm_coin_creator_vault_authority_pda(
    coin_creator: &Pubkey,
    program_id: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"creator_vault", coin_creator.as_ref()], program_id)
}

/// 派生 PumpAMM Fee Config PDA
/// 
/// 注意：根据 IDL，fee_config 的 seeds 包含一个固定的地址
pub fn derive_pump_amm_fee_config_pda(fee_program: &Pubkey) -> Result<(Pubkey, u8), PubkeyError> {
    // 根据 IDL，fee_config 的 seeds 是：
    // [b"fee_config", fixed_bytes]
    let fixed_bytes = [
        12, 20, 222, 252, 130, 94, 198, 118, 148, 37, 8, 24, 187, 101, 64, 101,
        244, 41, 141, 49, 86, 213, 113, 180, 212, 248, 9, 12, 24, 233, 168, 99,
    ];
    Ok(Pubkey::find_program_address(
        &[b"fee_config", &fixed_bytes],
        fee_program,
    ))
}

/// 派生 Pool Base Token Account PDA
pub fn derive_pool_base_token_account_pda(
    pool: &Pubkey,
    base_mint: &Pubkey,
) -> Pubkey {
    let associated_token_program_id: Pubkey = ASSOCIATED_TOKEN_PROGRAM_ID
        .parse()
        .expect("Invalid Associated Token Program ID");
    
    let (address, _bump) = Pubkey::find_program_address(
        &[pool.as_ref(), TOKEN_PROGRAM_ID.as_ref(), base_mint.as_ref()],
        &associated_token_program_id,
    );
    address
}

/// 派生 Pool Quote Token Account PDA
pub fn derive_pool_quote_token_account_pda(
    pool: &Pubkey,
    quote_mint: &Pubkey,
) -> Pubkey {
    let associated_token_program_id: Pubkey = ASSOCIATED_TOKEN_PROGRAM_ID
        .parse()
        .expect("Invalid Associated Token Program ID");
    
    let (address, _bump) = Pubkey::find_program_address(
        &[pool.as_ref(), TOKEN_PROGRAM_ID.as_ref(), quote_mint.as_ref()],
        &associated_token_program_id,
    );
    address
}

/// 派生 Protocol Fee Recipient Token Account PDA
pub fn derive_protocol_fee_recipient_token_account_pda(
    protocol_fee_recipient: &Pubkey,
    quote_mint: &Pubkey,
) -> Pubkey {
    let associated_token_program_id: Pubkey = ASSOCIATED_TOKEN_PROGRAM_ID
        .parse()
        .expect("Invalid Associated Token Program ID");
    
    let (address, _bump) = Pubkey::find_program_address(
        &[
            protocol_fee_recipient.as_ref(),
            TOKEN_PROGRAM_ID.as_ref(),
            quote_mint.as_ref(),
        ],
        &associated_token_program_id,
    );
    address
}

/// 派生 Coin Creator Vault ATA PDA
pub fn derive_coin_creator_vault_ata_pda(
    coin_creator_vault_authority: &Pubkey,
    quote_mint: &Pubkey,
) -> Pubkey {
    let associated_token_program_id: Pubkey = ASSOCIATED_TOKEN_PROGRAM_ID
        .parse()
        .expect("Invalid Associated Token Program ID");
    
    let (address, _bump) = Pubkey::find_program_address(
        &[
            coin_creator_vault_authority.as_ref(),
            TOKEN_PROGRAM_ID.as_ref(),
            quote_mint.as_ref(),
        ],
        &associated_token_program_id,
    );
    address
}

/// WSOL Mint (Wrapped SOL)
pub const WSOL_MINT: &str = "So11111111111111111111111111111111111111112";

/// 获取 WSOL Mint 地址
pub fn wsol_mint() -> Pubkey {
    WSOL_MINT.parse().expect("Invalid WSOL Mint")
}


/// USDC Mint
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

/// 获取 USDC Mint 地址
pub fn usdc_mint() -> Pubkey {
    USDC_MINT.parse().expect("Invalid USDC Mint")
}

/// 检查 mint 是否为 WSOL
pub fn is_wsol(mint: &Pubkey) -> bool {
    mint == &wsol_mint()
}

/// 检查 mint 是否为 USDC
pub fn is_usdc(mint: &Pubkey) -> bool {
    mint == &usdc_mint()
}

/// 检查 quote_mint 是否为 WSOL 或 USDC
pub fn is_wsol_or_usdc(quote_mint: &Pubkey) -> bool {
    is_wsol(quote_mint) || is_usdc(quote_mint)
}

/// OptionBool 类型（用于 track_volume 参数）
#[derive(Clone, Debug)]
pub enum OptionBool {
    None,
    Some(bool),
}

impl Default for OptionBool {
    fn default() -> Self {
        OptionBool::None
    }
}

impl OptionBool {
    /// 序列化为 Anchor 格式
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            OptionBool::None => vec![0],
            OptionBool::Some(true) => vec![1, 1],
            OptionBool::Some(false) => vec![1, 0],
        }
    }
}

