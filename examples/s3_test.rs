//! S3 集成测试
//! 
//! 测试与远程 MinIO 服务器的连接

use storage_backends::{S3Backend, StorageBackend};

/// 测试配置
const S3_ENDPOINT: &str = "http://10.126.126.30:9000";
const S3_BUCKET: &str = "synapse-test";
const S3_ACCESS_KEY: &str = "minioadmin";
const S3_SECRET_KEY: &str = "minio12345";

#[tokio::main]
async fn main() {
    println!("=== S3/MinIO 集成测试 ===\n");
    
    // 1. 创建 S3 后端
    println!("1. 创建 S3 后端连接...");
    let backend = match S3Backend::new(
        S3_ENDPOINT,
        S3_BUCKET,
        S3_ACCESS_KEY,
        S3_SECRET_KEY,
    ) {
        Ok(b) => {
            println!("   ✓ 连接成功");
            println!("   端点: {}", b.endpoint());
            println!("   存储桶: {}", b.bucket());
            b
        }
        Err(e) => {
            println!("   ✗ 连接失败: {}", e);
            return;
        }
    };
    
    // 2. 测试写入
    println!("\n2. 测试数据写入...");
    let test_key = "test/hello.txt";
    let test_data = b"Hello from SynapseCore!";
    
    match backend.save(test_key, test_data).await {
        Ok(_) => println!("   ✓ 写入成功: {} ({} bytes)", test_key, test_data.len()),
        Err(e) => {
            println!("   ✗ 写入失败: {}", e);
            return;
        }
    }
    
    // 3. 测试读取
    println!("\n3. 测试数据读取...");
    match backend.load(test_key).await {
        Ok(data) => {
            let content = String::from_utf8_lossy(&data);
            if data == test_data {
                println!("   ✓ 读取成功: {}", content);
            } else {
                println!("   ✗ 数据不匹配: {}", content);
            }
        }
        Err(e) => {
            println!("   ✗ 读取失败: {}", e);
            return;
        }
    }
    
    // 4. 测试列出文件
    println!("\n4. 测试文件列表...");
    match backend.list("test/").await {
        Ok(files) => {
            println!("   ✓ 找到 {} 个文件:", files.len());
            for file in &files {
                println!("     - {}", file);
            }
        }
        Err(e) => {
            println!("   ✗ 列出失败: {}", e);
        }
    }
    
    // 5. 测试文件大小
    println!("\n5. 测试文件大小...");
    match backend.size(test_key).await {
        Ok(size) => println!("   ✓ 文件大小: {} bytes", size),
        Err(e) => println!("   ✗ 获取大小失败: {}", e),
    }
    
    // 6. 测试文件存在性
    println!("\n6. 测试文件存在性...");
    match backend.exists(test_key).await {
        Ok(exists) => println!("   {} 文件存在: {}", if exists { "✓" } else { "✗" }, exists),
        Err(e) => println!("   ✗ 检查失败: {}", e),
    }
    
    // 7. 测试复制
    println!("\n7. 测试文件复制...");
    let copy_key = "test/hello_copy.txt";
    match backend.copy(test_key, copy_key).await {
        Ok(_) => println!("   ✓ 复制成功: {} -> {}", test_key, copy_key),
        Err(e) => println!("   ✗ 复制失败: {}", e),
    }
    
    // 8. 测试重命名
    println!("\n8. 测试文件重命名...");
    let rename_key = "test/hello_renamed.txt";
    match backend.rename(copy_key, rename_key).await {
        Ok(_) => println!("   ✓ 重命名成功: {} -> {}", copy_key, rename_key),
        Err(e) => println!("   ✗ 重命名失败: {}", e),
    }
    
    // 9. 测试删除
    println!("\n9. 测试文件删除...");
    match backend.delete(test_key).await {
        Ok(_) => println!("   ✓ 删除成功: {}", test_key),
        Err(e) => println!("   ✗ 删除失败: {}", e),
    }
    
    match backend.delete(rename_key).await {
        Ok(_) => println!("   ✓ 删除成功: {}", rename_key),
        Err(e) => println!("   ✗ 删除失败: {}", e),
    }
    
    // 10. 最终列表
    println!("\n10. 最终文件列表...");
    match backend.list("test/").await {
        Ok(files) => {
            println!("   剩余文件: {} 个", files.len());
            for file in &files {
                println!("     - {}", file);
            }
        }
        Err(e) => println!("   ✗ 列出失败: {}", e),
    }
    
    println!("\n=== 测试完成 ===");
}
