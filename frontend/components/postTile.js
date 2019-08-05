import React from "react";
import ReactMarkdown from 'react-markdown';
import Link from "next/link";

const layoutStyle = {
  margin: 20,
  padding: 20,
  border: '1px solid #DDD'
};

export default class PostTile extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      ...props.post
    };
  }

  render() {
    const { post_id, user, text, created_at, updated_at } = this.state;

    return (
      <div style={layoutStyle}>
        {user ?
          <Link href={`/user?id=${user.user_id}`}>
            <a>{user.username}</a>
          </Link> : 'Anonymous'
        }
        <ReactMarkdown source={text} escapeHtml={true}/>
      </div>
    )
  }
}
