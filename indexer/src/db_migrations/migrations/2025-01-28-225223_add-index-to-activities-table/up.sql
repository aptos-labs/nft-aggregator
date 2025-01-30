-- Your SQL goes here
CREATE INDEX idx_activities_nft_id on activities (nft_id);

CREATE INDEX idx_activities_nft_name on activities (nft_name);

CREATE INDEX idx_activities_collection_addr on activities (collection_addr);

CREATE INDEX idx_activities_collection_creator_addr on activities (collection_creator_addr);

CREATE INDEX idx_activities_collection_name on activities (collection_name);