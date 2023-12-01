import * as ReactDOM from 'react-dom/client';
import { ApolloClient, InMemoryCache, ApolloProvider, HttpLink,  } from '@apollo/client';
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";
import App from './App';
import LogIn from './pages/LogIn';
import SignUp from './pages/SignUp';
import PostPage from './pages/PostPage';
import NavBar from './pages/components/Navbar';
import { Link } from "react-router-dom";

import './index.css';

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
  },
  {
    path: "/login",
    element: <LogIn/>,
  },
  {
    path: "/signup",
    element: <SignUp/>,
  },
  {
    path: "/post",
    element: <PostPage/>,
  },
]);

const client = new ApolloClient({
  cache: new InMemoryCache({addTypename: false}),
  link: new HttpLink({
    uri: "http://localhost:4000/graphql",
    fetchOptions: {
      mode: 'cors',
    },
  })
});

const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);

root.render(
  <ApolloProvider client={client}>
    <RouterProvider router={router} />
  </ApolloProvider>,
);