export function timeZoneOffset ({ServerDate}: {ServerDate: Date}) {
    const localTime: string = 
    new Date(
        ServerDate.getTime() - ServerDate.getTimezoneOffset() * 60000
    ).toISOString().slice(0,16)

    return (
        localTime
    )
}