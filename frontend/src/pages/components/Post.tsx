
type Person = {
    username: string;
    title: string;
    content: string;
    location: string;
}
const Post = (person: Person) => {
    return(
        <div className="h-118 w-full rounded-lg object-cover">
        <h2 className="mt-4 text-2xl font-semibold capitalize text-gray-800 dark:text-white">{person.title}</h2>
        <span className="mt-2 text-lg uppercase tracking-wider text-[#000] dark:text-blue-400">{person.username} living in {person.location}</span>
        <p className="mt-3 text-md uppercase tracking-wider text-blue-500 dark:text-blue-400">{person.content}</p>
        </div>
    )
}

export default Post;