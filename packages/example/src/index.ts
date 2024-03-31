/**
 * Welcome to Cloudflare Workers! This is your first worker.
 *
 * - Run `npm run dev` in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your worker in action
 * - Run `npm run deploy` to publish your worker
 *
 * Learn more at https://developers.cloudflare.com/workers/
 */

declare global {
	interface Env {
		OVERRIDDEN_ENV: string;
		TEST_DO: DurableObject;
	}
}

export default {
	async fetch(
		_request: Request,
		_env: Env,
		_ctx: ExecutionContext,
	): Promise<Response> {
		return new Response('Hello World!');
	},
};
