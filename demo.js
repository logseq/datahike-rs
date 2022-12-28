
// import * as datahike from "./datahike.darwin-x64";
const datahike = require('./datahike.darwin-arm64.node')


const main = async () => {
    console.log(await datahike.init());


    const config = "{:store {:backend :file :path \"./path-to-db\"} :schema-flexibility :read}";

    const exists = await datahike.databaseExists(config);
    console.log(exists);


    if (!exists) {
        console.log(await datahike.createDatabase(config));
    }

    let tx_data = "[{:age 42}]"

    console.log(await datahike.transact(config, tx_data));


    const q = "[:find ?a . :where [?e :age ?a]]"

    await datahike.query(q, [["db", config]])

}


main().then(console.log)
