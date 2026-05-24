export function TimeZoneOffset ({ServerDate}: {ServerDate: Date}) {
    const LocalTime: string = 
    new Date(
        new Date(ServerDate).getTime() - new Date(ServerDate).getTimezoneOffset() * 60000
    ).toISOString().slice(0,16)

    return (
        LocalTime
    )
}