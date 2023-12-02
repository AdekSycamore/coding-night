type Person = {
  username: string;
  title: string;
  content: string;
  location: string;
};
const Post = (person: Person) => {
  return (
    <div>
     
          <div className="max-w-sm bg-white px-6 pt-6 pb-2 rounded-xl shadow-lg transform hover:scale-105 transition duration-500">
            <h3 className="mb-3 text-xl font-bold text-indigo-600">
              {person.username}
            </h3>
            <div className="relative">
              <img
                className="w-full rounded-xl"
                src="https://warsawtour.pl/wp-content/uploads/2023/03/Panorama-Warszawy-z-Widok-Towers-fot.-Filip-Kwiatkowski-1.jpg"
                alt="Colors"
              />
            </div>
            <h1 className="mt-4 text-gray-800 text-2xl font-bold cursor-pointer">
              {person.title}
            </h1>
            <div className="my-4">
              <div className="flex space-x-1 items-center m-2">
                <span className="material-symbols-outlined">location_on</span>
                <p>{person.location}</p>
              </div>
              <div className="flex space-x-1 items-center m-2">
              <span className="material-symbols-outlined">chat</span>
                <p className="w-full whitespace-nowrap text-ellipsis overflow-hidden">{person.content}</p>
              </div>
              <button className="mt-4 text-xl w-full text-white bg-primary py-2 rounded-xl shadow-lg">
                Show offer!
              </button>
            </div>
          </div>
        </div>
  );
};

export default Post;
