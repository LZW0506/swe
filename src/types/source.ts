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
    Name: string,
    Engine: string,
    Version: string,
    Row_format: string,
    Rows: string,
    Create_time?: string,
    Collation: string,
    Comment: string,
}