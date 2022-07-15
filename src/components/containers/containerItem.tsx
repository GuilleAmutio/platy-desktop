
type Props = {
    container: {name: string, state: string}
}

const containerItem = ({container}: Props) => {
    return (<li>
        <span>{container.name}</span>
        <span>{container.state}</span>
    </li>)
}

export default containerItem