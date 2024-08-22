use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    // 默认情况下，build.rs 得到的中间文件保存在 out_dir 环境指定的目录中
    // 如果没有明确设置 out_dir 环境变量，则默认为 cargo 的构建目录，target
    //
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();

    builder
        .out_dir("src/pb") // 输出的路径，此处指定为项目 src/pb
        // .with_serde(
        //     &["User"],
        //     true,
        //     true,
        //     Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        // )
        // .with_sqlx_from_row(&["User"], None)
        // .with_derive_builder(
        //     &[
        //         "User",
        //         "QueryRequest",
        //         "RawQueryRequest",
        //         "TimeQuery",
        //         "IdQuery",
        //     ],
        //     None
        // )
        //
        // 指定要编译的 proto 文件路径列表，第二个参数是提供protobuf的扩展路径
        // 因为 protobuf 官方提供了一些扩展功能，自己也可能会写一些扩展功能，
        // 如存在，则指定扩展文件路径，如果没有，则指定为proto文件所在目录即可
        .compile(
            &[
                "../protos/metadata/messages.proto",
                "../protos/metadata/rpc.proto",
            ],
            &["../protos"],
        )?; // import "metadata/messages.proto";//todo 需要在 protos 下面去找 metadata，如果将 protos换成
            // protos/metadata，就会找不到依赖的目录
            // &["../protos/user-stats"])?;// import "messages.proto";

    Ok(())
}
