// import * as datahike from "./datahike.darwin-x64";
const datahike = require('./datahike.darwin-arm64.node')


const main = () => {
    datahike.init()

    const config = `{:store {:backend :file :path "./path-to-db"} :schema-flexibility :read}`

    const exists = datahike.databaseExists(config);
    console.log("EX:", exists);

    if (!exists) {
        datahike.createDatabase(config)
    }
    // schema
    datahike.transact(config, `
        [{:db/ident :block/uuid
              :db/valueType :db.type/uuid
              :db/cardinality :db.cardinality/one
              :db/unique :db.unique/identity }
             {:db/ident :block/parent
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one
              :db/index true}
             {:db/ident :block/left
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one
              :db/index true}
             {:db/ident :block/collapsed?
              :db/valueType :db.type/boolean
              :db/cardinality :db.cardinality/one
              :db/index true}
             {:db/ident :block/format
              :db/valueType :db.type/keyword
              :db/cardinality :db.cardinality/one}
             {:db/ident :block/page
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one
              :db/index true}
             {:db/ident :block/refs
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/many}
             {:db/ident :block/path-refs
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/many}
             {:db/ident :block/tags
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/many}
             {:db/ident :block/alias
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/many}
             {:db/ident :block/name
              :db/valueType :db.type/string
              :db/unique :db.unique/identity
              :db/cardinality :db.cardinality/one}
             {:db/ident :block/original-name
              :db/valueType :db.type/string
              :db/unique :db.unique/identity
              :db/cardinality :db.cardinality/one}
             {:db/ident :block/namespace
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one}
             {:db/ident :block/macros
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/many}
             {:db/ident :block/file
              :db/valueType :db.type/ref
              :db/cardinality :db.cardinality/one}
             {:db/ident :file/path
              :db/valueType :db.type/string
              :db/cardinality :db.cardinality/one
              :db/unique :db.unique/identity}]
    `)


    // let tx_data = "[{:age 42}]"
    /* let tx_data = `[{:name  "Alice", :age   20 }
     {:name  "Bob", :age   30 }
     {:name  "Charlie", :age   40 }
     {:age 15 }]`; */
    let tx_data = `[{:block/name "jan 5th, 2023",
    :block/original-name "Jan 5th, 2023",
    :block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570",
    :block/created-at 1672900419536,
    :block/updated-at 1672900419536,
    :block/journal? true,
    :block/journal-day 20230105,
    :block/format :markdown}]`

    console.log(datahike.transact(config, tx_data));

    let retract_tx_data = `([:db/retract 19 :block/properties-text-values]
      [:db/retract 19 :block/alias]
      [:db/retract 19 :block/warning]
      [:db/retract 19 :block/pre-block?]
      [:db/retract 19 :block/priority]
      [:db/retract 19 :block/invalid-properties]
      [:db/retract 19 :block/properties]
      [:db/retract 19 :block/updated-at]
      [:db/retract 19 :block/repeated?]
      [:db/retract 19 :block/refs]
      [:db/retract 19 :block/scheduled]
      [:db/retract 19 :block/properties-order]
      [:db/retract 19 :block/created-at]
      [:db/retract 19 :block/deadline]
      [:db/retract 19 :block/tags]
      [:db/retract 19 :block/path-refs]
      [:db/retract 19 :block/type]
      [:db/retract 19 :block/marker]
      {:db/id 18, :block/updated-at 1672928886369}
      {:block/uuid #uuid "63b69aea-33ee-45e9-acb1-83cdca2b9ed9",
       :block/properties {},
       :block/journal? true,
       :block/left {:db/id 18},
       :block/refs (),
       :block/journal-day 20230105,
       :block/format :markdown,
       :block/tags (),
       :block/content "nice",
       :db/id 19,
       :block/macros (),
       :block/path-refs (),
       :block/parent {:db/id 18},
       :block/unordered true,
       :block/page {:db/id 18}})`

    console.log(datahike.transact(config, retract_tx_data));


    const q = `[:find ?e ?n ?a
        :where
        [?e :block/name ?n]
        [?e :block/created-at ?a]]`

    datahike.query(q, [["db", config]])

    console.log("Ent:", datahike.entity(config, 2))
    console.log("Pull:", datahike.pull(config, "[*]", 3))
    console.log("Pull by lookup ref:", datahike.pull(config, "[*]", `[:block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570"]`))
    console.log("Pull:", datahike.pull(config, "[:block/name :block/format]", 2))

    console.log("Pull many:", datahike.pullMany(config, "[:block/name]", `[1 2 3 4 [:block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570"]]`))

    console.log("Pull with default:", datahike.pull(config, `[(default :foo "bar")]`, 2))


    console.log("datoms:", datahike.datoms(config, ":eavt", 1, 4, 100))

    console.log("schema:", datahike.schema(config))

    console.log("Ent:", datahike.entity(config, `[:block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570"]`))

    datahike.deleteDatabase(config)
}


main()
