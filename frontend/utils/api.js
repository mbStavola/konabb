const fetch = require('isomorphic-unfetch');

const baseUrl = process.env.BACKEND_URL ? process.env.BACKEND_URL : 'http://localhost:8080';

// Users
export const loginUser = async (username, password) => {
  const options = {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password })
  };

  const response = await fetch(`${baseUrl}/user/login`, options);
  return await response.json();
};

export const getUser = async (userId) => {
  const response = await fetch(`${baseUrl}/user/${userId}`);
  return await response.json();
};

export const createUser = async (user) => {
  const options = {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(user)
  };
  const response = await fetch(`${baseUrl}/user`, options);
  return await response.json();
};

export const updateUser = async () => {
  const options = {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(user)
  };
  const response = await fetch(`${baseUrl}/user`, { method: 'PUT' });
  return await response.json();
};

// Threads
export const getThread = async (threadId) => {
  const response = await fetch(`${baseUrl}/thread/${threadId}`);
  return await response.json();
};

export const createThread = async (thread) => {
  const options = {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(thread),
  };
  const response = await fetch(`${baseUrl}/thread`, options);
  return await response.text();
};

// Posts
export const createPost = async (post) => {
  const options = {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    credentials: 'include',
    body: JSON.stringify(post)
  };
  await fetch(`${baseUrl}/post`, options);
};

// Boards
export const listBoards = async () => {
  const response = await fetch(`${baseUrl}/boards`);
  return await response.json();
};

export const getBoard = async (shortName) => {
  const response = await fetch(`${baseUrl}/board/${shortName}`);
  return await response.json();
};
