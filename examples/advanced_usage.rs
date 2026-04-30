// SynapseCore - 高级使用示例

use data_core::{DataEntity, DataType, Cipher, MetadataBuilder};
use uuid::Uuid;
use chrono::Utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SynapseCore 高级使用示例 ===\n");

    // 1. 使用密码派生密钥
    println!("1. 使用密码派生密钥");
    let password = b"my-strong-password-123";
    let salt = b"unique-salt-value-";
    let cipher = Cipher::from_password(password, salt)?;
    println!("   ✓ 从密码派生密钥成功\n");

    // 2. 使用关联数据加密
    println!("2. 使用关联数据加密");
    let plaintext = b"敏感数据";
    let aad = b"用户ID:12345";
    let ciphertext = cipher.encrypt(plaintext, Some(aad))?;
    let decrypted = cipher.decrypt(&ciphertext, Some(aad))?;
    println!("   关联数据: {}", String::from_utf8_lossy(aad));
    println!("   解密验证: {}\n", plaintext.to_vec() == decrypted);

    // 3. 批量创建数据实体
    println!("3. 批量创建数据实体");
    let owner_id = Uuid::new_v4();
    let mut entities = Vec::new();

    for i in 0..5 {
        let data = format!("数据项 {}", i);
        let ciphertext = cipher.encrypt(data.as_bytes(), None)?;
        let entity = DataEntity::new(
            owner_id,
            DataType::Config,
            ciphertext,
        );
        entities.push(entity);
    }
    println!("   创建了 {} 个数据实体\n", entities.len());

    // 4. 数据分类
    println!("4. 数据分类");
    let categories = vec![
        DataType::Credential,
        DataType::Config,
        DataType::File,
        DataType::Contact,
        DataType::Generic,
    ];
    for category in &categories {
        println!("   - {}: {}", category, category.as_str());
    }
    println!();

    // 5. 元数据管理
    println!("5. 元数据管理");
    let metadata = MetadataBuilder::new()
        .source("advanced-example")
        .size(1024)
        .mime_type("application/json")
        .priority(90)
        .favorite(true)
        .property("env", "production")
        .property("version", "1.0.0")
        .build();
    println!("   源: {:?}", metadata.source);
    println!("   大小: {:?} 字节", metadata.size);
    println!("   优先级: {}", metadata.priority);
    println!("   收藏: {}\n", metadata.is_favorite);

    // 6. 版本控制
    println!("6. 版本控制");
    let mut entity = entities[0].clone();
    println!("   初始版本: {}", entity.version);
    for i in 0..3 {
        let new_data = format!("更新 {}", i);
        let new_ciphertext = cipher.encrypt(new_data.as_bytes(), None)?;
        entity.update_content(new_ciphertext);
        println!("   更新后版本: {}", entity.version);
    }
    println!();

    // 7. 数据导出
    println!("7. 数据导出");
    let json_export = entity.to_json()?;
    let msgpack_export = entity.to_msgpack()?;
    println!("   JSON 导出: {} 字符", json_export.len());
    println!("   MessagePack 导出: {} 字节\n", msgpack_export.len());

    // 8. 错误处理
    println!("8. 错误处理");
    let invalid_data = b"短数据";
    let short_ciphertext = &invalid_data[..5]; // 故意截断
    match cipher.decrypt(short_ciphertext, None) {
        Ok(_) => println!("   解密成功"),
        Err(e) => println!("   解密失败: {}", e),
    }
    println!();

    // 9. 安全最佳实践
    println!("9. 安全最佳实践");
    println!("   ✓ 使用强密码 (12+ 字符)");
    println!("   ✓ 使用唯一的 salt 值");
    println!("   ✓ 定期轮换密钥");
    println!("   ✓ 使用关联数据保护完整性");
    println!("   ✓ 实现密钥包装 (WrappedKey)");

    println!("\n=== 高级示例完成 ===");

    Ok(())
}
