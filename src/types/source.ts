export interface FormState {
    sourceType: string,
    username: string,
    password: string,
    address: {
        url: string,
        port: number | String,
        icon: string
    },
    database: string,
    name: string,
    uuid?: string
}

export interface tableInfoType {
    name: string,
    engine: string,
    // Version: string,
    row_format: string,
    rows: string,
    create_time: string,
    collation: string,
    comment: string,
}