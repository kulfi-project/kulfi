impl kulfi::Identity {
    pub async fn read(
        _path: &std::path::Path,
        id: String,
        client_pools: kulfi_utils::HttpConnectionPools,
    ) -> eyre::Result<Self> {
        Self::from_id52(id.as_str(), client_pools)
    }
}
