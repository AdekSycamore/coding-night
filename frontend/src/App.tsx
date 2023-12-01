import { useQuery, gql } from '@apollo/client';
import './App.css';

const GET_LOCATIONS = gql`
query{
  allTodos {
    id
    task
    done
  }
}
`;

export default function App() {
  const { loading, error, data } = useQuery(GET_LOCATIONS);

  if (loading) return <p>Loading...</p>;
  if (error) return <h1 className="text-3xl font-bold underline">{error.message}</h1>;

  return (
    <div>
      <h1 className="text-3xl font-bold underline">
        {JSON.stringify(data)}
      </h1>
    </div>
  );
}