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



  return (
    <div>
      <h1 className="text-3xl font-bold underline">
        fdfdgdfgfddfhfgfghhfgcscsdfgdfgf
      </h1>
    </div>
  );
}