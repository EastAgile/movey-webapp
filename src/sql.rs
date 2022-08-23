use diesel::sql_types::Text;

sql_function!(fn lower(x: Text) -> Text);
