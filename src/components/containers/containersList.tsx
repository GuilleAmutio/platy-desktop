import { PropsWithChildren } from "react"

const ContainersList = ({ children }: PropsWithChildren<{}>) => {
    return <ul>
        {children}
    </ul>
}

export default ContainersList;