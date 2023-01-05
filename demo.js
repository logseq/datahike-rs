
// import * as datahike from "./datahike.darwin-x64";
const datahike = require('./datahike.darwin-arm64.node')


const main = async () => {
    await datahike.init()

    const config = `{:store {:backend :file :path "./path-to-db"} :schema-flexibility :read}`

    const exists = await datahike.databaseExists(config);
    console.log("EX:", exists);

    if (!exists) {
        await datahike.createDatabase(config)
    }
    // schema
    await datahike.transact(config, `
        [{
            :db/ident       :block/uuid
            :db/valueType   :db.type/uuid
            :db/cardinality :db.cardinality/one
            :db/unique      :db.unique/identity
        }]
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

    console.log(await datahike.transact(config, tx_data));


    const q = `[:find ?e ?n ?a
        :where
        [?e :block/name ?n]
        [?e :block/created-at ?a]]`

    await datahike.query(q, [["db", config]])

    console.log("Ent:", await datahike.entity(config, 2))
    console.log("Pull:", await datahike.pull(config, "[*]", 3))
    console.log("Pull by lookup ref:", await datahike.pull(config, "[*]", `[:block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570"]`))
    console.log("Pull:", await datahike.pull(config, "[:block/name :block/format]", 2))

    console.log("Pull many:", await datahike.pullMany(config, "[:block/name]", `[1 2 3 4 [:block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570"]]`))

    console.log("Pull with default:", await datahike.pull(config, `[(default :foo "bar")]`, 2))


    console.log("datoms:", await datahike.datoms(config, ":eavt", 1, 4, 100))

    console.log("schema:", await datahike.schema(config))

    console.log("Ent:", await datahike.entity(config, `[:block/uuid #uuid "63b66f43-e10f-4cf7-8c02-e0d4b09c1570"]`))

    await datahike.deleteDatabase(config)
}


main().catch(console.error)
