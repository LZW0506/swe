import localforage from "localforage";

export const sourceDb = localforage.createInstance({
    name: "source",
    description: "数据源库"
});
