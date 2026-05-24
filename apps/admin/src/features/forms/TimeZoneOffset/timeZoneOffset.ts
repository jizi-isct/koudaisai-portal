export function timeZoneOffset ({serverDate}: {serverDate: Date}) {
    const localTime: string = 
    new Date(
        serverDate.getTime() - serverDate.getTimezoneOffset() * 60000
    ).toISOString().slice(0,16)

    return (
        localTime
    )
}