
// import * as datahike from "./datahike.darwin-x64";
const datahike = require('./datahike.darwin-arm64.node')


const main = async () => {
    console.log(await datahike.init());
    const config = "{:store {:backend :file :path \"./path-to-db\"} :schema-flexibility :read}";


    const exists = await datahike.databaseExists(config);
    console.log("EX:", exists);


    if (!exists) {
        console.log(await datahike.createDatabase(config));
    }

    // let tx_data = "[{:age 42}]"
    let tx_data = `[{:name  "Alice", :age   20 }
    {:name  "Bob", :age   30 }
    {:name  "Charlie", :age   40 }
    {:age 15 }]`;

    console.log(await datahike.transact(config, tx_data));


    const q = `[:find ?e ?n ?a
        :where
        [?e :name ?n]
        [?e :age ?a]]`

    await datahike.query(q, [["db", config]])

    console.log("Ent:", await datahike.entity(config, 2))
    console.log("Pull:", await datahike.pull(config, "[:name :age]", 2))

    console.log("Pull many:", await datahike.pullMany(config, "[:name]", "[1 2 3 4]"))


    console.log(await datahike.deleteDatabase(config));

}


main().then(console.log)
