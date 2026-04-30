// SynapseCore - 基本使用示例

use data_core::{DataEntity, DataType, Cipher, MetadataBuilder};
use uuid::Uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SynapseCore 基本使用示例 ===\n");

    // 1. 创建加密器
    println!("1. 创建加密器");
    let cipher = Cipher::new()?;
    println!("   ✓ 加密器创建成功\n");

    // 2. 加密数据
    println!("2. 加密数据");
    let plaintext = b"这是我的密码: MySecretPassword123";
    let ciphertext = cipher.encrypt(plaintext, None)?;
    println!("   明文长度: {} 字节", plaintext.len());
    println!("   密文长度: {} 字节\n", ciphertext.len());

    // 3. 解密数据
    println!("3. 解密数据");
    let decrypted = cipher.decrypt(&ciphertext, None)?;
    println!("   解密结果: {}\n", String::from_utf8_lossy(&decrypted));

    // 4. 创建数据实体
    println!("4. 创建数据实体");
    let owner_id = Uuid::new_v4();
    let entity = DataEntity::new(
        owner_id,
        DataType::Credential,
        ciphertext,
    );
    println!("   数据ID: {}", entity.id);
    println!("   数据类型: {}", entity.data_type);
    println!("   版本: {}\n", entity.version);

    // 5. 添加元数据
    println!("5. 添加元数据");
    let metadata = MetadataBuilder::new()
        .source("example-app")
        .size(entity.size() as u64)
        .mime_type("application/octet-stream")
        .priority(80)
        .favorite(true)
        .property("category", "password")
        .property("service", "github")
        .build();
    println!("   元数据: {}\n", metadata);

    // 6. 序列化
    println!("6. 序列化");
    let json = entity.to_json()?;
    println!("   JSON 长度: {} 字符", json.len());

    let msgpack = entity.to_msgpack()?;
    println!("   MessagePack 长度: {} 字节\n", msgpack.len());

    // 7. 反序列化
    println!("7. 反序列化");
    let restored = DataEntity::from_json(&json)?;
    println!("   恢复的数据ID: {}", restored.id);
    println!("   数据匹配: {}\n", entity.id == restored.id);

    // 8. 更新数据
    println!("8. 更新数据");
    let mut entity = entity;
    let new_ciphertext = cipher.encrypt(b"新密码: NewPassword456", None)?;
    entity.update_content(new_ciphertext);
    println!("   更新后版本: {}\n", entity.version);

    // 9. 软删除
    println!("9. 软删除");
    entity.soft_delete();
    println!("   已删除: {}", entity.is_deleted);
    println!("   当前版本: {}\n", entity.version);

    // 10. 安全提示
    println!("10. 安全特性");
    println!("   ✓ AES-256-GCM 认证加密");
    println!("   ✓ 每次加密使用唯一 Nonce");
    println!("   ✓ 支持关联数据 (AEAD)");
    println!("   ✓ 密钥派生使用 HKDF");
    println!("   ✓ 软删除支持审计追踪");

    println!("\n=== 示例完成 ===");

    Ok(())
}
