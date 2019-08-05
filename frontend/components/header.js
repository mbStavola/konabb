import Link from 'next/link';
import UserTile from "./userTile";
import { LoginTile } from "./loginTile";

import { withCookies } from 'react-cookie';

import jwtDecode from 'jwt-decode';
import moment from 'moment';

const SESSION_NAME = 'konabb-session';

const linkStyle = {
  marginRight: 15
};

const createSessionTimeout = (target, cookies, expirationTime) => {
  setTimeout(() => {
    target.setState({ user: null, session: null });
    cookies.remove(SESSION_NAME);
  }, expirationTime);
};

class Header extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      user: null,
      session: null
    }
  }

  async componentDidMount() {
    const { cookies } = this.props;

    const session = cookies.get(SESSION_NAME);
    if (!session) {
      return;
    }

    const claims = jwtDecode(session);

    const expirationTime = moment(claims.iat).add(claims.exp, 'seconds');
    if (claims && moment().isAfter(expirationTime)) {
      cookies.remove(SESSION_NAME);
      return;
    }

    createSessionTimeout(this, cookies, claims.exp * 1000);

    this.setState({
      session: {
        userId: claims.uid,
        expirationTime
      }
    });
  }

  async handleLogin(userSession) {
    const { cookies } = this.props;
    const expirationTime = moment().add(userSession.expires_in, 'seconds');

    createSessionTimeout(this, cookies, userSession.expires_in * 1000);

    cookies.set(SESSION_NAME, userSession.access_token, { maxAge: userSession.expires_in });

    this.setState({
      user: { user_id: userSession.userId },
      expirationTime,
      session: {
        userId: userSession.user_id,
        expirationTime
      }
    });
  }

  render() {
    const { user, session } = this.state;
    return (
      <div>
        <span>KonaBB</span>
        <Link href="/">
          <a style={linkStyle}>Home</a>
        </Link>
        {
          session ? <UserTile userId={session.userId} onFetch={(user) => this.setState({ user })}/> :
            <LoginTile onLogin={(userSession) => this.handleLogin(userSession)}/>
        }
      </div>
    );
  }
}

export default withCookies(Header);
