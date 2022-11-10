#[derive(thiserror::Error, Debug)]
// エラーのタイプ2つをenumで定義
pub enum ApiError {
    #[error("Post not found")]
    NotFound,
    #[error(transparent)]
    Other(anyhow::Error),
}

// 使うクレートから帰る可能性のあるエラー型からOtherに変換するFromトレイトを実装
// Fromを実装しておくと?記述時に自動的に適用される
macro_rules! impl_from_trait {
    ($etype:ty) => {
        impl From<$etype> for ApiError {
            fn from(e: $etype) -> Self {
                ApiError::Other(anyhow::anyhow!(e))
            }
        }
    };
}
