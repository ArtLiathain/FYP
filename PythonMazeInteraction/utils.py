import itertools
from matplotlib import pyplot as plt
import numpy as np
import math
import torch

class PrioritizedReplayBuffer:
    def __init__(self, capacity, state_shape, alpha=0.6):
        self.capacity = capacity
        self.state_shape = state_shape
        self.alpha = alpha  # prioritization exponent

        # Main buffers
        self.states = np.empty((capacity, *state_shape), dtype=np.float32)
        self.actions = np.empty((capacity,), dtype=np.int64)
        self.rewards = np.empty((capacity,), dtype=np.float32)
        self.next_states = np.empty((capacity, *state_shape), dtype=np.float32)
        self.dones = np.empty((capacity,), dtype=bool)
        self.truncateds = np.empty((capacity,), dtype=bool)

        # Priority buffer
        self.priorities = np.zeros((capacity,), dtype=np.float32)

        self.size = 0
        self.ptr = 0
        self.epsilon = 1e-6  # small value to avoid zero priority

    def append(self, state, action, reward, next_state, done, truncated):
        max_prio = self.priorities.max() if self.size > 0 else 1.0

        self.states[self.ptr] = state
        self.actions[self.ptr] = action
        self.rewards[self.ptr] = reward
        self.next_states[self.ptr] = next_state
        self.dones[self.ptr] = done
        self.truncateds[self.ptr] = truncated

        self.priorities[self.ptr] = max_prio  # assign max priority for new samples

        self.ptr = (self.ptr + 1) % self.capacity
        self.size = min(self.size + 1, self.capacity)

    def sample(self, batch_size, beta=0.4):
        if self.size == self.capacity:
            prios = self.priorities
        else:
            prios = self.priorities[:self.size]

        # Sampling probabilities
        probs = prios ** self.alpha
        probs /= probs.sum()

        idxs = np.random.choice(self.size, batch_size, p=probs)

        # Importance-sampling weights
        weights = (self.size * probs[idxs]) ** (-beta)
        weights /= weights.max()  # normalize for stability

        batch = dict(
            state=self.states[idxs],
            action=self.actions[idxs],
            reward=self.rewards[idxs],
            next_state=self.next_states[idxs],
            done=self.dones[idxs],
            truncated=self.truncateds[idxs],
            weights=weights,
            indices=idxs
        )

        return batch

    def update_priorities(self, indices, td_errors):
        # Use absolute TD error + small epsilon to avoid zero priorities
        self.priorities[indices] = np.abs(td_errors) + self.epsilon

    def __len__(self):
        return self.size

def plot_success_rate(report_cards):
    success_rates = [card["score"]["success_rate_in_exploitation"] for card in report_cards]
    plt.plot(success_rates)
    plt.title("Success Rate in Exploitation Over Time")
    plt.xlabel("Episode")
    plt.ylabel("Success Rate")
    plt.grid(True)
    plt.show()
    
    
def moving_average(data, window_size=3):
    return np.convolve(data, np.ones(window_size)/window_size, mode='valid')

def plot_episode_returns(all_explore, all_exploit, window_size):
    plt.figure(figsize=(12, 5))
    plt.plot(moving_average(all_explore, window_size), label="Explore Episode Reward", alpha=0.7)
    plt.plot(moving_average(all_exploit, window_size), label="Exploit Episode Reward", alpha=0.7)
    plt.xlabel("Episode")
    plt.ylabel("Total Reward")
    plt.title("Episode Return Over Time")
    plt.legend()
    plt.grid(True)
    plt.tight_layout()
    plt.show()
    
def plot_losses(losses):
    # Safely convert all tensors to detached NumPy scalars
    processed_losses = [loss.detach().cpu().item() if torch.is_tensor(loss) else loss for loss in losses]

    plt.plot(processed_losses)
    plt.title("Training Loss Over Time")
    plt.xlabel("Training Step")
    plt.ylabel("Loss")
    plt.grid(True)
    plt.show()

def plot_q_histogram(q_values, title="Q-Value Distribution"):
    plt.figure(figsize=(8, 5))
    plt.hist(q_values, bins=50, color="orchid", edgecolor="black")
    plt.title(title)
    plt.xlabel("Q-value")
    plt.ylabel("Frequency")
    plt.grid(True)
    plt.tight_layout()
    plt.show()
    
def plot_q_values(q_value_log):
    plt.figure(figsize=(10, 5))
    plt.plot(q_value_log)
    plt.title("Q-Values Over Time")
    plt.xlabel("Training Steps")
    plt.ylabel("Q-Value")
    plt.grid(True)
    plt.show()
    
def generate_param_combinations(grid):
    keys = list(grid.keys())
    values = list(grid.values())
    for combination in itertools.product(*values):
        yield dict(zip(keys, combination))
        
# import pandas as pd
# csv_data = []
# for i, result in enumerate(results):
#     param_row = {**result["params"]}
#     param_row["run_id"] = i
#     param_row["mean_reward"] = sum(result["rewards"]) / len(result["rewards"])
#     param_row["max_reward"] = max(result["rewards"])
#     param_row["min_reward"] = min(result["rewards"])
#     csv_data.append(param_row)

# df = pd.DataFrame(csv_data)
# df.to_csv("results_log2.csv", index=False)
# print("Saved results to results_log.csv")
def get_changed_hyperparams(defaults, current):
    return {
        k: v for k, v in current.items()
        if k not in defaults or defaults[k] != v
    }

def parameterised_results_display(results, shared_params):
    cols = 2
    rows = math.ceil(len(results) / cols)

    fig, axs = plt.subplots(rows, cols, figsize=(12, rows * 4))

    if len(results) == 1:
        axs = [axs]

    # Step 1: Find the max absolute reward for symmetric y-axis
    max_reward = max(
        max(abs(min(result["rewards"])), abs(max(result["rewards"])))
        for result in results
    )

    # Round up to make the graph cleaner
    y_limit = math.ceil(max_reward)

    for i, result in enumerate(results):
        ax = axs[i // cols][i % cols] if rows > 1 else axs[i % cols]

        rewards = result["rewards"]
        changed = get_changed_hyperparams(shared_params, result["params"])

        ax.plot(rewards)
        ax.set_title(f"Run {i} - Changed Params:")
        ax.set_xlabel("Episode")
        ax.set_ylabel("Reward")
        ax.grid(True)

        # Step 2: Set a constant center of 0
        ax.set_ylim(-y_limit, y_limit)

        # Annotate changed hyperparameters
        annotation = "\n".join([f"{k}: {v}" for k, v in changed.items()])
        ax.text(1.01, 0.5, annotation, transform=ax.transAxes,
                fontsize=9, verticalalignment='center',
                bbox=dict(facecolor='white', edgecolor='gray', boxstyle='round,pad=0.5'))

    # Hide any unused subplots
    for j in range(i + 1, rows * cols):
        fig.delaxes(axs[j // cols][j % cols] if rows > 1 else axs[j % cols])

    fig.tight_layout()
    plt.savefig("comparison_plot2.png")
    plt.show()