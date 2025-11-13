use solana_pump_grpc_sdk::{TradeClient, OptionBool};
use solana_sdk::pubkey::Pubkey;

/// 交易指令构建示例
/// 
/// 这个示例展示了如何使用 TradeClient 构建 Pump 和 PumpAMM 的交易指令
/// 
/// 注意：这个示例只构建指令，不发送交易
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 交易指令构建示例 ===\n");

    // ========================================
    // Pump (Bonding Curve) 交易示例
    // ========================================
    println!("1. Pump (Bonding Curve) 交易示例");
    println!("--------------------------------");

    // 创建 Pump 交易客户端
    let pump_client = TradeClient::new();

    // 示例地址（实际使用时需要替换为真实的地址）
    let user = "11111111111111111111111111111111"
        .parse::<Pubkey>()
        .unwrap();
    let mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        .parse::<Pubkey>()
        .unwrap();

    // 1.1 普通模式买入指令
    println!("\n1.1 普通模式买入指令");
    let buy_ix_normal = pump_client.build_buy_instruction(
        &user,
        &mint,
        1000000,                    // amount: 买入的代币数量
        100000000,                  // max_sol_cost: 最大 SOL 成本（lamports）
        OptionBool::Some(true),     // track_volume: 跟踪交易量
        false,                      // is_mayhem_mode: 普通模式
    )?;
    println!("✅ 普通模式买入指令构建成功");
    println!("   Program ID: {}", buy_ix_normal.program_id);
    println!("   账户数量: {}", buy_ix_normal.accounts.len());
    println!("   指令数据长度: {} bytes", buy_ix_normal.data.len());

    // 1.2 Mayhem 模式买入指令
    println!("\n1.2 Mayhem 模式买入指令");
    let buy_ix_mayhem = pump_client.build_buy_instruction(
        &user,
        &mint,
        1000000,                    // amount: 买入的代币数量
        100000000,                  // max_sol_cost: 最大 SOL 成本（lamports）
        OptionBool::Some(true),     // track_volume: 跟踪交易量
        true,                       // is_mayhem_mode: Mayhem 模式
    )?;
    println!("✅ Mayhem 模式买入指令构建成功");
    println!("   Program ID: {}", buy_ix_mayhem.program_id);
    println!("   账户数量: {}", buy_ix_mayhem.accounts.len());
    println!("   指令数据长度: {} bytes", buy_ix_mayhem.data.len());

    // 1.3 普通模式卖出指令
    println!("\n1.3 普通模式卖出指令");
    let sell_ix_normal = pump_client.build_sell_instruction(
        &user,
        &mint,
        1000000,                    // amount: 卖出的代币数量
        95000000,                   // min_sol_output: 最小 SOL 输出（lamports）
        false,                      // is_mayhem_mode: 普通模式
    )?;
    println!("✅ 普通模式卖出指令构建成功");
    println!("   Program ID: {}", sell_ix_normal.program_id);
    println!("   账户数量: {}", sell_ix_normal.accounts.len());
    println!("   指令数据长度: {} bytes", sell_ix_normal.data.len());

    // 1.4 Mayhem 模式卖出指令
    println!("\n1.4 Mayhem 模式卖出指令");
    let sell_ix_mayhem = pump_client.build_sell_instruction(
        &user,
        &mint,
        1000000,                    // amount: 卖出的代币数量
        95000000,                   // min_sol_output: 最小 SOL 输出（lamports）
        true,                       // is_mayhem_mode: Mayhem 模式
    )?;
    println!("✅ Mayhem 模式卖出指令构建成功");
    println!("   Program ID: {}", sell_ix_mayhem.program_id);
    println!("   账户数量: {}", sell_ix_mayhem.accounts.len());
    println!("   指令数据长度: {} bytes", sell_ix_mayhem.data.len());

    // ========================================
    // PumpAMM 交易示例
    // ========================================
    println!("\n\n2. PumpAMM 交易示例");
    println!("--------------------------------");

    // 创建 PumpAMM 交易客户端
    let amm_client = TradeClient::pump_amm();

    // 示例地址（实际使用时需要替换为真实的地址）
    let pool = "11111111111111111111111111111111"
        .parse::<Pubkey>()
        .unwrap();
    let base_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        .parse::<Pubkey>()
        .unwrap();
    let quote_mint = "So11111111111111111111111111111111111111112"
        .parse::<Pubkey>()
        .unwrap(); // WSOL
    let coin_creator = "11111111111111111111111111111111"
        .parse::<Pubkey>()
        .unwrap();
    let protocol_fee_recipient = "11111111111111111111111111111111"
        .parse::<Pubkey>()
        .unwrap();

    // 2.1 买入指令（quote_mint 是 WSOL）
    println!("\n2.1 买入指令（quote_mint = WSOL）");
    let amm_buy_ix = amm_client.build_pump_amm_buy_instruction(
        &user,
        &pool,
        &base_mint,
        &quote_mint,               // WSOL
        &coin_creator,
        &protocol_fee_recipient,
        1000000,                   // base_amount_out: 期望买入的 base token 数量
        100000000,                 // max_quote_amount_in: 最大 quote token 输入
        OptionBool::Some(true),    // track_volume: 跟踪交易量
        false,                     // is_mayhem_mode: 普通模式
    )?;
    println!("✅ PumpAMM 买入指令构建成功");
    println!("   Program ID: {}", amm_buy_ix.program_id);
    println!("   账户数量: {} (包含 volume accumulator)", amm_buy_ix.accounts.len());
    println!("   指令数据长度: {} bytes", amm_buy_ix.data.len());
    println!("   指令类型: Buy (因为 quote_mint 是 WSOL)");

    // 2.2 卖出指令（quote_mint 是 WSOL）
    println!("\n2.2 卖出指令（quote_mint = WSOL）");
    let amm_sell_ix = amm_client.build_pump_amm_sell_instruction(
        &user,
        &pool,
        &base_mint,
        &quote_mint,               // WSOL
        &coin_creator,
        &protocol_fee_recipient,
        1000000,                   // base_amount_in: 卖出的 base token 数量
        95000000,                  // min_quote_amount_out: 最小 quote token 输出
        false,                     // is_mayhem_mode: 普通模式
    )?;
    println!("✅ PumpAMM 卖出指令构建成功");
    println!("   Program ID: {}", amm_sell_ix.program_id);
    println!("   账户数量: {} (不包含 volume accumulator)", amm_sell_ix.accounts.len());
    println!("   指令数据长度: {} bytes", amm_sell_ix.data.len());
    println!("   指令类型: Sell (因为 quote_mint 是 WSOL)");

    // 2.3 买入指令（quote_mint 不是 WSOL/USDC，会自动使用反向交易）
    println!("\n2.3 买入指令（quote_mint 不是 WSOL/USDC，自动使用反向交易）");
    let non_wsol_quote_mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
        .parse::<Pubkey>()
        .unwrap(); // 假设这是非 WSOL/USDC 的 quote_mint
    let amm_buy_ix_reverse = amm_client.build_pump_amm_buy_instruction(
        &user,
        &pool,
        &base_mint,
        &non_wsol_quote_mint,      // 不是 WSOL/USDC
        &coin_creator,
        &protocol_fee_recipient,
        1000000,                   // base_amount_out
        100000000,                 // max_quote_amount_in
        OptionBool::Some(true),    // track_volume
        false,                     // is_mayhem_mode
    )?;
    println!("✅ PumpAMM 反向买入指令构建成功");
    println!("   Program ID: {}", amm_buy_ix_reverse.program_id);
    println!("   账户数量: {} (包含 volume accumulator)", amm_buy_ix_reverse.accounts.len());
    println!("   指令数据长度: {} bytes", amm_buy_ix_reverse.data.len());
    println!("   指令类型: Sell (反向，因为 quote_mint 不是 WSOL/USDC)");

    // 2.4 Mayhem 模式买入指令
    println!("\n2.4 Mayhem 模式买入指令");
    let amm_buy_ix_mayhem = amm_client.build_pump_amm_buy_instruction(
        &user,
        &pool,
        &base_mint,
        &quote_mint,               // WSOL
        &coin_creator,
        &protocol_fee_recipient,
        1000000,                   // base_amount_out
        100000000,                 // max_quote_amount_in
        OptionBool::Some(true),    // track_volume
        true,                      // is_mayhem_mode: Mayhem 模式
    )?;
    println!("✅ PumpAMM Mayhem 模式买入指令构建成功");
    println!("   Program ID: {}", amm_buy_ix_mayhem.program_id);
    println!("   账户数量: {}", amm_buy_ix_mayhem.accounts.len());
    println!("   指令数据长度: {} bytes", amm_buy_ix_mayhem.data.len());
    println!("   使用 Mayhem Fee Recipient");

    // ========================================
    // OptionBool 使用示例
    // ========================================
    println!("\n\n3. OptionBool 使用示例");
    println!("--------------------------------");
    println!("OptionBool::None: {:?}", OptionBool::None.to_bytes());
    println!("OptionBool::Some(true): {:?}", OptionBool::Some(true).to_bytes());
    println!("OptionBool::Some(false): {:?}", OptionBool::Some(false).to_bytes());

    println!("\n✅ 所有指令构建完成！");
    println!("\n注意：");
    println!("  - 这些指令仅用于演示，实际使用时需要替换为真实的地址");
    println!("  - 这些指令需要添加到交易中并签名后才能发送到链上");
    println!("  - PumpAMM 会根据 quote_mint 是否为核心资产（WSOL/USDC）自动选择指令类型");

    Ok(())
}

