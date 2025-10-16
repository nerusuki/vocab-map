export default async function authMiddleware({ request, context }, next) {
  return await next();
}
