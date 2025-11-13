use crate::error::Result;
use crate::trading::helpers::{
    ASSOCIATED_TOKEN_PROGRAM_ID, pump_amm_program_id, FEE_RECIPIENT, MAYHEM_FEE_RECIPIENT, *,
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// 交易客户端
/// 
/// 用于构建买入和卖出指令
pub struct TradeClient {
    program_id: Pubkey,
}

impl TradeClient {
    /// 创建新的交易客户端（用于 Pump 程序）
    pub fn new() -> Self {
        Self {
            program_id: pump_program_id(),
        }
    }

    /// 使用自定义程序 ID 创建交易客户端
    pub fn with_program_id(program_id: Pubkey) -> Self {
        Self { program_id }
    }

    /// 创建 PumpAMM 交易客户端
    pub fn pump_amm() -> Self {
        Self {
            program_id: pump_amm_program_id(),
        }
    }
}

impl Default for TradeClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TradeClient {
    /// 构建买入指令
    /// 
    /// # 参数
    /// 
    /// * `user` - 买入用户（signer）
    /// * `mint` - 代币 mint 地址
    /// * `amount` - 买入的代币数量
    /// * `max_sol_cost` - 最大 SOL 成本
    /// * `track_volume` - 是否跟踪交易量
    /// * `is_mayhem_mode` - 是否为 mayhem mode（影响 fee recipient 和 token program）
    pub fn build_buy_instruction(
        &self,
        user: &Pubkey,
        mint: &Pubkey,
        amount: u64,
        max_sol_cost: u64,
        track_volume: OptionBool,
        is_mayhem_mode: bool,
    ) -> Result<Instruction> {
        // 派生所有需要的账户
        let (global, _global_bump) = derive_global_pda(&self.program_id);
        let fee_recipient = get_fee_recipient(is_mayhem_mode);
        let token_program = get_token_program_id(is_mayhem_mode);
        let (bonding_curve, _bonding_bump) = derive_bonding_curve_pda(mint, &self.program_id);
        let associated_bonding_curve = derive_associated_bonding_curve(&bonding_curve, mint);
        let associated_user = derive_user_associated_token_account(user, mint);
        let (creator_vault, _creator_bump) = derive_creator_vault_pda(&fee_recipient, &self.program_id);
        let (event_authority, _event_bump) = derive_event_authority_pda(&self.program_id);
        let (global_volume_accumulator, _global_vol_bump) =
            derive_global_volume_accumulator_pda(&self.program_id);
        let (user_volume_accumulator, _user_vol_bump) =
            derive_user_volume_accumulator_pda(user, &self.program_id);
        let (fee_config, _fee_config_bump) = derive_fee_config_pda(&fee_recipient, &fee_program_id())?;

        // 构建指令数据
        // discriminator: [102, 6, 61, 18, 1, 218, 235, 234]
        let mut instruction_data = vec![102, 6, 61, 18, 1, 218, 235, 234];
        
        // 添加参数：amount (u64), max_sol_cost (u64), track_volume (OptionBool)
        instruction_data.extend_from_slice(&amount.to_le_bytes());
        instruction_data.extend_from_slice(&max_sol_cost.to_le_bytes());
        instruction_data.extend_from_slice(&track_volume.to_bytes());

        // 构建账户列表
        let accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(fee_recipient, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(*user, true), // signer
            AccountMeta::new_readonly(Pubkey::new_from_array([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]), false), // System Program: 11111111111111111111111111111111
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(self.program_id, false),
            AccountMeta::new(global_volume_accumulator, false),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(fee_config, false),
            AccountMeta::new_readonly(fee_program_id(), false),
        ];

        Ok(Instruction {
            program_id: self.program_id,
            accounts,
            data: instruction_data,
        })
    }

    /// 构建卖出指令
    /// 
    /// # 参数
    /// 
    /// * `user` - 卖出用户（signer）
    /// * `mint` - 代币 mint 地址
    /// * `amount` - 卖出的代币数量
    /// * `min_sol_output` - 最小 SOL 输出
    /// * `is_mayhem_mode` - 是否为 mayhem mode（影响 fee recipient 和 token program）
    pub fn build_sell_instruction(
        &self,
        user: &Pubkey,
        mint: &Pubkey,
        amount: u64,
        min_sol_output: u64,
        is_mayhem_mode: bool,
    ) -> Result<Instruction> {
        // 派生所有需要的账户
        let (global, _global_bump) = derive_global_pda(&self.program_id);
        let fee_recipient = get_fee_recipient(is_mayhem_mode);
        let token_program = get_token_program_id(is_mayhem_mode);
        let (bonding_curve, _bonding_bump) = derive_bonding_curve_pda(mint, &self.program_id);
        let associated_bonding_curve = derive_associated_bonding_curve(&bonding_curve, mint);
        let associated_user = derive_user_associated_token_account(user, mint);
        let (creator_vault, _creator_bump) = derive_creator_vault_pda(&fee_recipient, &self.program_id);
        let (event_authority, _event_bump) = derive_event_authority_pda(&self.program_id);
        let (fee_config, _fee_config_bump) = derive_fee_config_pda(&fee_recipient, &fee_program_id())?;

        // 构建指令数据
        // discriminator: [51, 230, 133, 164, 1, 127, 131, 173]
        let mut instruction_data = vec![51, 230, 133, 164, 1, 127, 131, 173];
        
        // 添加参数：amount (u64), min_sol_output (u64)
        instruction_data.extend_from_slice(&amount.to_le_bytes());
        instruction_data.extend_from_slice(&min_sol_output.to_le_bytes());

        // 构建账户列表
        let accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(fee_recipient, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(*user, true), // signer
            AccountMeta::new_readonly(Pubkey::new_from_array([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]), false), // System Program: 11111111111111111111111111111111
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(self.program_id, false),
            AccountMeta::new_readonly(fee_config, false),
            AccountMeta::new_readonly(fee_program_id(), false),
        ];

        Ok(Instruction {
            program_id: self.program_id,
            accounts,
            data: instruction_data,
        })
    }

    /// 构建 PumpAMM 买入指令
    /// 
    /// # 参数
    /// 
    /// * `user` - 买入用户（signer）
    /// * `pool` - Pool PDA 地址（或使用 derive_pump_amm_pool_pda 计算）
    /// * `base_mint` - Base token mint 地址
    /// * `quote_mint` - Quote token mint 地址（通常是 WSOL）
    /// * `coin_creator` - Coin creator 地址（从 Pool 账户中读取）
    /// * `protocol_fee_recipient` - 协议手续费接收地址（从 GlobalConfig 中读取）
    /// * `base_amount_out` - 期望买入的 base token 数量
    /// * `max_quote_amount_in` - 最大 quote token 输入
    /// * `track_volume` - 是否跟踪交易量
    /// * `is_mayhem_mode` - 是否为 mayhem mode（影响 fee recipient）
    /// 
    /// # 注意
    /// 
    /// 这个方法需要从链上读取 Pool 账户数据来获取 coin_creator 等信息。
    /// 如果已经知道这些值，可以直接使用这个方法构建指令。
    /// 
    /// 如果 quote_mint 是 WSOL/USDC，使用买入指令。
    /// 如果 quote_mint 不是 WSOL/USDC，使用卖出指令（反向交易）。
    pub fn build_pump_amm_buy_instruction(
        &self,
        user: &Pubkey,
        pool: &Pubkey,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
        coin_creator: &Pubkey,
        protocol_fee_recipient: &Pubkey,
        base_amount_out: u64,
        max_quote_amount_in: u64,
        track_volume: OptionBool,
        is_mayhem_mode: bool,
    ) -> Result<Instruction> {
        let program_id = pump_amm_program_id();
        let (global_config, _global_bump) = derive_pump_amm_global_config_pda(&program_id);
        let user_base_token_account = derive_user_associated_token_account(user, base_mint);
        let user_quote_token_account = derive_user_associated_token_account(user, quote_mint);
        let pool_base_token_account = derive_pool_base_token_account_pda(pool, base_mint);
        let pool_quote_token_account = derive_pool_quote_token_account_pda(pool, quote_mint);
        
        // 根据 mayhem mode 选择 fee recipient
        let fee_recipient = if is_mayhem_mode {
            MAYHEM_FEE_RECIPIENT
        } else {
            FEE_RECIPIENT
        };
        
        let protocol_fee_recipient_token_account =
            derive_protocol_fee_recipient_token_account_pda(protocol_fee_recipient, quote_mint);
        let (event_authority, _event_bump) = derive_event_authority_pda(&program_id);
        let (coin_creator_vault_authority, _vault_authority_bump) =
            derive_pump_amm_coin_creator_vault_authority_pda(coin_creator, &program_id);
        let coin_creator_vault_ata =
            derive_coin_creator_vault_ata_pda(&coin_creator_vault_authority, quote_mint);
        let (fee_config, _fee_config_bump) = derive_pump_amm_fee_config_pda(&fee_program_id())?;

        // 检查 quote_mint 是否为 WSOL/USDC
        let quote_is_wsol_or_usdc = is_wsol_or_usdc(quote_mint);

        // 根据 quote_is_wsol_or_usdc 决定使用哪个 discriminator
        let (discriminator, data_param1, data_param2) = if quote_is_wsol_or_usdc {
            // 买入指令：base_amount_out, max_quote_amount_in
            ([102, 6, 61, 18, 1, 218, 235, 234], base_amount_out, max_quote_amount_in)
        } else {
            // 卖出指令（反向）：base_amount_in, min_quote_amount_out
            ([51, 230, 133, 164, 1, 127, 131, 173], max_quote_amount_in, base_amount_out)
        };

        // 构建指令数据
        let mut instruction_data = Vec::with_capacity(24);
        instruction_data.extend_from_slice(&discriminator);
        instruction_data.extend_from_slice(&data_param1.to_le_bytes());
        instruction_data.extend_from_slice(&data_param2.to_le_bytes());
        if quote_is_wsol_or_usdc {
            // 买入指令需要 track_volume 参数
            instruction_data.extend_from_slice(&track_volume.to_bytes());
        }

        // 构建账户列表
        let mut accounts = Vec::with_capacity(23);
        accounts.extend([
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user, true), // signer
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(user_base_token_account, false),
            AccountMeta::new(user_quote_token_account, false),
            AccountMeta::new(pool_base_token_account, false),
            AccountMeta::new(pool_quote_token_account, false),
            AccountMeta::new_readonly(fee_recipient, false),
            AccountMeta::new(protocol_fee_recipient_token_account, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // base_token_program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // quote_token_program
            AccountMeta::new_readonly(Pubkey::new_from_array([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]), false), // System Program: 11111111111111111111111111111111
            AccountMeta::new_readonly(
                ASSOCIATED_TOKEN_PROGRAM_ID.parse().expect("Invalid ATA Program ID"),
                false,
            ),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(coin_creator_vault_ata, false),
            AccountMeta::new_readonly(coin_creator_vault_authority, false),
        ]);
        
        // 仅当 quote_is_wsol_or_usdc 时添加 volume accumulator
        if quote_is_wsol_or_usdc {
            let (global_volume_accumulator, _global_vol_bump) =
                derive_global_volume_accumulator_pda(&program_id);
            let (user_volume_accumulator, _user_vol_bump) =
                derive_user_volume_accumulator_pda(user, &program_id);
            accounts.push(AccountMeta::new(global_volume_accumulator, false));
            accounts.push(AccountMeta::new(user_volume_accumulator, false));
        }
        
        accounts.push(AccountMeta::new_readonly(fee_config, false));
        accounts.push(AccountMeta::new_readonly(fee_program_id(), false));

        Ok(Instruction {
            program_id,
            accounts,
            data: instruction_data,
        })
    }

    /// 构建 PumpAMM 卖出指令
    /// 
    /// # 参数
    /// 
    /// * `user` - 卖出用户（signer）
    /// * `pool` - Pool PDA 地址
    /// * `base_mint` - Base token mint 地址
    /// * `quote_mint` - Quote token mint 地址（通常是 WSOL）
    /// * `coin_creator` - Coin creator 地址（从 Pool 账户中读取）
    /// * `protocol_fee_recipient` - 协议手续费接收地址（从 GlobalConfig 中读取）
    /// * `base_amount_in` - 卖出的 base token 数量
    /// * `min_quote_amount_out` - 最小 quote token 输出
    /// * `is_mayhem_mode` - 是否为 mayhem mode（影响 fee recipient）
    /// 
    /// # 注意
    /// 
    /// 如果 quote_mint 是 WSOL/USDC，使用卖出指令。
    /// 如果 quote_mint 不是 WSOL/USDC，使用买入指令（反向交易）。
    pub fn build_pump_amm_sell_instruction(
        &self,
        user: &Pubkey,
        pool: &Pubkey,
        base_mint: &Pubkey,
        quote_mint: &Pubkey,
        coin_creator: &Pubkey,
        protocol_fee_recipient: &Pubkey,
        base_amount_in: u64,
        min_quote_amount_out: u64,
        is_mayhem_mode: bool,
    ) -> Result<Instruction> {
        let program_id = pump_amm_program_id();
        let (global_config, _global_bump) = derive_pump_amm_global_config_pda(&program_id);
        let user_base_token_account = derive_user_associated_token_account(user, base_mint);
        let user_quote_token_account = derive_user_associated_token_account(user, quote_mint);
        let pool_base_token_account = derive_pool_base_token_account_pda(pool, base_mint);
        let pool_quote_token_account = derive_pool_quote_token_account_pda(pool, quote_mint);
        
        // 根据 mayhem mode 选择 fee recipient
        let fee_recipient = if is_mayhem_mode {
            MAYHEM_FEE_RECIPIENT
        } else {
            FEE_RECIPIENT
        };
        
        let protocol_fee_recipient_token_account =
            derive_protocol_fee_recipient_token_account_pda(protocol_fee_recipient, quote_mint);
        let (event_authority, _event_bump) = derive_event_authority_pda(&program_id);
        let (coin_creator_vault_authority, _vault_authority_bump) =
            derive_pump_amm_coin_creator_vault_authority_pda(coin_creator, &program_id);
        let coin_creator_vault_ata =
            derive_coin_creator_vault_ata_pda(&coin_creator_vault_authority, quote_mint);
        let (fee_config, _fee_config_bump) = derive_pump_amm_fee_config_pda(&fee_program_id())?;

        // 检查 quote_mint 是否为 WSOL/USDC
        let quote_is_wsol_or_usdc = is_wsol_or_usdc(quote_mint);

        // 根据 quote_is_wsol_or_usdc 决定使用哪个 discriminator
        let (discriminator, data_param1, data_param2) = if quote_is_wsol_or_usdc {
            // 卖出指令：base_amount_in, min_quote_amount_out
            ([51, 230, 133, 164, 1, 127, 131, 173], base_amount_in, min_quote_amount_out)
        } else {
            // 买入指令（反向）：base_amount_out, max_quote_amount_in
            ([102, 6, 61, 18, 1, 218, 235, 234], min_quote_amount_out, base_amount_in)
        };

        // 构建指令数据
        let mut instruction_data = Vec::with_capacity(24);
        instruction_data.extend_from_slice(&discriminator);
        instruction_data.extend_from_slice(&data_param1.to_le_bytes());
        instruction_data.extend_from_slice(&data_param2.to_le_bytes());
        if !quote_is_wsol_or_usdc {
            // 反向买入指令需要 track_volume 参数（使用 None，因为卖出不需要跟踪）
            instruction_data.extend_from_slice(&OptionBool::None.to_bytes());
        }

        // 构建账户列表
        let mut accounts = Vec::with_capacity(23);
        accounts.extend([
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user, true), // signer
            AccountMeta::new_readonly(global_config, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(user_base_token_account, false),
            AccountMeta::new(user_quote_token_account, false),
            AccountMeta::new(pool_base_token_account, false),
            AccountMeta::new(pool_quote_token_account, false),
            AccountMeta::new_readonly(fee_recipient, false),
            AccountMeta::new(protocol_fee_recipient_token_account, false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // base_token_program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // quote_token_program
            AccountMeta::new_readonly(Pubkey::new_from_array([
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ]), false), // System Program: 11111111111111111111111111111111
            AccountMeta::new_readonly(
                ASSOCIATED_TOKEN_PROGRAM_ID.parse().expect("Invalid ATA Program ID"),
                false,
            ),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(coin_creator_vault_ata, false),
            AccountMeta::new_readonly(coin_creator_vault_authority, false),
        ]);
        
        // 仅当 quote 不是 WSOL/USDC 时添加 volume accumulator（反向交易）
        if !quote_is_wsol_or_usdc {
            let (global_volume_accumulator, _global_vol_bump) =
                derive_global_volume_accumulator_pda(&program_id);
            let (user_volume_accumulator, _user_vol_bump) =
                derive_user_volume_accumulator_pda(user, &program_id);
            accounts.push(AccountMeta::new(global_volume_accumulator, false));
            accounts.push(AccountMeta::new(user_volume_accumulator, false));
        }
        
        accounts.push(AccountMeta::new_readonly(fee_config, false));
        accounts.push(AccountMeta::new_readonly(fee_program_id(), false));

        Ok(Instruction {
            program_id,
            accounts,
            data: instruction_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_global_pda() {
        let program_id = pump_program_id();
        let (pda, bump) = derive_global_pda(&program_id);
        assert_eq!(bump, 255); // 通常 bump 是 255
    }

    #[test]
    fn test_derive_bonding_curve_pda() {
        use std::str::FromStr;
        let program_id = pump_program_id();
        let mint = Pubkey::from_str("11111111111111111111111111111111").unwrap();
        let (pda, _bump) = derive_bonding_curve_pda(&mint, &program_id);
        // PDA 应该不同
        assert_ne!(pda, mint);
    }

    #[test]
    fn test_option_bool_serialization() {
        assert_eq!(OptionBool::None.to_bytes(), vec![0]);
        assert_eq!(OptionBool::Some(true).to_bytes(), vec![1, 1]);
        assert_eq!(OptionBool::Some(false).to_bytes(), vec![1, 0]);
    }
}

