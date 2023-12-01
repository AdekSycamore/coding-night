
type Person = {
    username: string;
    title: string;
    content: string;
    location: string;
}
const Post = (person: Person) => {
    return(
    <div className="w-1/6 h-50 p-9 pr-9 m-4 overflow-hidden text-black bg-white">
        <div> <span>{person.username}: {person.location}</span>
        <h2>{person.title}</h2></div>
        <div><p>{person.content}</p></div>
    </div>
    )
}

export default Post;