# rstock

This crate consists of two parts:

- A daemon that scrapes the web for price data and collects it into a database.

- An API that offers access to the collected price data.

You need to set the following environment variable:

`DATABASE_URL=postgres://<USER>:<PASSWORD>@<IP ADDRESS>/<DATABASE>`

You also need to fill the 'watchlist' table with ISINs that you want to track.
