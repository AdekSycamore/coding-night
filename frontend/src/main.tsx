import * as ReactDOM from 'react-dom/client';
import { ApolloClient, InMemoryCache, ApolloProvider, HttpLink,  } from '@apollo/client';
import App from './App';
import './index.css';

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
    <App />
  </ApolloProvider>,
);