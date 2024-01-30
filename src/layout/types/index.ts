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
    name: string
}
