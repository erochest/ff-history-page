table! {
    moz_anno_attributes (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    moz_annos (id) {
        id -> Nullable<Integer>,
        place_id -> Integer,
        anno_attribute_id -> Nullable<Integer>,
        mime_type -> Nullable<Text>,
        content -> Nullable<Text>,
        flags -> Nullable<Integer>,
        expiration -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        dateAdded -> Nullable<BigInt>,
        lastModified -> Nullable<BigInt>,
    }
}

table! {
    moz_bookmarks (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        fk -> Nullable<Integer>,
        parent -> Nullable<Integer>,
        position -> Nullable<Integer>,
        title -> Nullable<Text>,
        keyword_id -> Nullable<Integer>,
        folder_type -> Nullable<Text>,
        dateAdded -> Nullable<BigInt>,
        lastModified -> Nullable<BigInt>,
        guid -> Nullable<Text>,
        syncStatus -> Integer,
        syncChangeCounter -> Integer,
    }
}

table! {
    moz_bookmarks_deleted (guid) {
        guid -> Nullable<Text>,
        dateRemoved -> BigInt,
    }
}

table! {
    moz_historyvisits (id) {
        id -> Nullable<Integer>,
        from_visit -> Nullable<Integer>,
        place_id -> Nullable<Integer>,
        visit_date -> Nullable<BigInt>,
        visit_type -> Nullable<Integer>,
        session -> Nullable<Integer>,
    }
}

table! {
    moz_hosts (id) {
        id -> Nullable<Integer>,
        host -> Text,
        frecency -> Nullable<Integer>,
        typed -> Integer,
        prefix -> Nullable<Text>,
    }
}

table! {
    moz_inputhistory (place_id, input) {
        place_id -> Integer,
        input -> Text,
        use_count -> Nullable<Integer>,
    }
}

table! {
    moz_items_annos (id) {
        id -> Nullable<Integer>,
        item_id -> Integer,
        anno_attribute_id -> Nullable<Integer>,
        mime_type -> Nullable<Text>,
        content -> Nullable<Text>,
        flags -> Nullable<Integer>,
        expiration -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Integer>,
        dateAdded -> Nullable<BigInt>,
        lastModified -> Nullable<BigInt>,
    }
}

table! {
    moz_keywords (id) {
        id -> Nullable<Integer>,
        keyword -> Nullable<Text>,
        place_id -> Nullable<Integer>,
        post_data -> Nullable<Text>,
    }
}

table! {
    moz_meta (key) {
        key -> Text,
        value -> Binary,
    }
}

table! {
    moz_origins (id) {
        id -> Nullable<Integer>,
        prefix -> Text,
        host -> Text,
        frecency -> Integer,
    }
}

table! {
    moz_places (id) {
        id -> Nullable<Integer>,
        url -> Nullable<Text>,
        title -> Nullable<Text>,
        rev_host -> Nullable<Text>,
        visit_count -> Nullable<Integer>,
        hidden -> Integer,
        typed -> Integer,
        favicon_id -> Nullable<Integer>,
        frecency -> Integer,
        last_visit_date -> Nullable<BigInt>,
        guid -> Nullable<Text>,
        foreign_count -> Integer,
        url_hash -> Integer,
        description -> Nullable<Text>,
        preview_image_url -> Nullable<Text>,
        origin_id -> Nullable<Integer>,
    }
}

joinable!(moz_places -> moz_origins (origin_id));

allow_tables_to_appear_in_same_query!(
    moz_anno_attributes,
    moz_annos,
    moz_bookmarks,
    moz_bookmarks_deleted,
    moz_historyvisits,
    moz_hosts,
    moz_inputhistory,
    moz_items_annos,
    moz_keywords,
    moz_meta,
    moz_origins,
    moz_places,
);
