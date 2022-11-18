import { useEffect, useState } from "react";
import getToken from "../utils/getToken";

const App = () => {
  const [token, setToken] = useState<null | string>(null);
  useEffect(() => {
    getToken().then(setToken);
  }, []);

  if (!token) {
    return <div>Loading...</div>;
  }

  return (
    <div className="flex flex-row text-white">
      Token is: {token}
    </div>
  );
}

export default App;
