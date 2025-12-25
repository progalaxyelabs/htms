# Example: Instagram Clone

A simplified Instagram clone with feed, post detail, and profile pages.

## Features

- ✅ Feed with posts and stories
- ✅ Like and bookmark posts
- ✅ Post detail with comments
- ✅ User profile with posts grid
- ✅ Bottom navigation
- ✅ Responsive design

## Complete Source Code

### app.htms

```htms
component BottomNav {
  nav [class: "bottom-nav"] {
    a [
      href: "#/",
      class: ctx.currentRoute == "/" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-home"]
      span {{ Home }}
    }

    a [
      href: "#/search",
      class: ctx.currentRoute == "/search" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-search"]
      span {{ Search }}
    }

    a [
      href: "#/add",
      class: ctx.currentRoute == "/add" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-plus"]
      span {{ Add }}
    }

    a [
      href: "#/profile",
      class: ctx.currentRoute == "/profile" ? "nav-item active" : "nav-item"
    ] {
      i [class: "icon-user"]
      span {{ Profile }}
    }
  }
}

component StoryItem(item: story) {
  div [
    class: story.seen ? "story-item seen" : "story-item",
    onClick: viewStory(story.id)
  ] {
    img [src: story.user.avatar, alt: story.user.username]
    span [class: "story-username"] {
      {{ ${story.user.username} }}
    }
  }
}

component FeedPost(item: post) {
  article [class: "feed-post"] {
    header [class: "post-header"] {
      div [class: "post-user"] {
        img [src: post.user.avatar, class: "avatar", alt: post.user.username]
        div {
          h3 [class: "username"] {
            {{ ${post.user.username} }}
          }
          span [class: "location"] @if(post.location) {
            {{ ${post.location} }}
          }
        }
      }
      button [class: "btn-more", onClick: showPostOptions(post.id)] {
        i [class: "icon-more"]
      }
    }

    img [src: post.image, class: "post-image", alt: "Post image"]

    div [class: "post-actions"] {
      div [class: "actions-left"] {
        button [onClick: toggleLike(post.id)] {
          i [class: post.isLiked ? "icon-heart-filled" : "icon-heart"]
        }
        button [onClick: navigateToPost(post.id)] {
          i [class: "icon-comment"]
        }
        button [onClick: sharePost(post.id)] {
          i [class: "icon-share"]
        }
      }
      button [onClick: toggleBookmark(post.id)] {
        i [class: post.isBookmarked ? "icon-bookmark-filled" : "icon-bookmark"]
      }
    }

    div [class: "post-info"] {
      div [class: "likes"] @if(post.likesCount > 0) {
        strong {{ ${post.likesCount} likes }}
      }

      div [class: "caption"] {
        strong {{ ${post.user.username} }}
        span {{ ${post.caption} }}
      }

      a @if(post.commentsCount > 0) [
        href: "#/posts/${post.id}",
        class: "view-comments"
      ] {
        {{ View all ${post.commentsCount} comments }}
      }

      time [class: "timestamp"] {
        {{ ${post.timeAgo} }}
      }
    }
  }
}

component Comment(item: comment) {
  div [class: "comment"] {
    img [src: comment.user.avatar, class: "avatar-sm"]
    div [class: "comment-content"] {
      div [class: "comment-text"] {
        strong {{ ${comment.user.username} }}
        span {{ ${comment.text} }}
      }
      div [class: "comment-meta"] {
        span {{ ${comment.timeAgo} }}
        button [onClick: replyToComment(comment.id), class: "btn-text"] {
          {{ Reply }}
        }
        span @if(comment.likesCount > 0) {
          {{ ${comment.likesCount} likes }}
        }
      }
    }
    button [onClick: toggleCommentLike(comment.id), class: "btn-icon"] {
      i [class: comment.isLiked ? "icon-heart-filled" : "icon-heart"]
    }
  }
}

page feed "/" {
  div [class: "app-container"] {
    header [class: "app-header"] {
      h1 [class: "logo"] {{ Instagram }}
      button [onClick: openMessages, class: "btn-icon"] {
        i [class: "icon-message"]
      }
    }

    section [class: "stories"] {
      div [class: "stories-scroll"] @for(ctx.stories as story) {
        StoryItem(item: story)
      }
    }

    main [class: "feed"] {
      div @if(ctx.isLoadingPosts) [class: "loading"] {
        p {{ Loading... }}
      } @else {
        div @for(ctx.posts as post) {
          FeedPost(item: post)
        }
      }

      div @if(ctx.posts.length == 0 && !ctx.isLoadingPosts) [class: "empty-feed"] {
        p {{ No posts yet. Start following people! }}
      }
    }

    BottomNav
  }
}

page postDetail "/posts/:id" {
  div [class: "app-container"] {
    header [class: "detail-header"] {
      button [onClick: goBack, class: "btn-back"] {
        i [class: "icon-back"]
      }
      h2 {{ Post }}
    }

    main [class: "post-detail"] @if(ctx.currentPost) {
      img [src: ctx.currentPost.image, class: "detail-image"]

      div [class: "detail-content"] {
        div [class: "post-actions"] {
          div [class: "actions-left"] {
            button [onClick: toggleLike(ctx.currentPost.id)] {
              i [class: ctx.currentPost.isLiked ? "icon-heart-filled" : "icon-heart"]
            }
            span [class: "likes-count"] {
              {{ ${ctx.currentPost.likesCount} likes }}
            }
          }
          button [onClick: toggleBookmark(ctx.currentPost.id)] {
            i [class: ctx.currentPost.isBookmarked ? "icon-bookmark-filled" : "icon-bookmark"]
          }
        }

        section [class: "comments-section"] {
          div [class: "comment original-caption"] {
            img [src: ctx.currentPost.user.avatar, class: "avatar-sm"]
            div [class: "comment-content"] {
              div [class: "comment-text"] {
                strong {{ ${ctx.currentPost.user.username} }}
                span {{ ${ctx.currentPost.caption} }}
              }
              div [class: "comment-meta"] {
                span {{ ${ctx.currentPost.timeAgo} }}
              }
            }
          }

          div @for(ctx.comments as comment) {
            Comment(item: comment)
          }

          div @if(ctx.isLoadingComments) [class: "loading"] {
            p {{ Loading comments... }}
          }
        }
      }

      footer [class: "comment-input-bar"] {
        img [src: ctx.currentUser.avatar, class: "avatar-sm"]
        input [
          type: "text",
          bind: ctx.newComment,
          placeholder: "Add a comment...",
          class: "comment-input"
        ]
        button [
          onClick: submitComment,
          disabled: ctx.newComment.length == 0,
          class: "btn-post"
        ] {
          {{ Post }}
        }
      }
    } @else {
      div [class: "loading"] {
        p {{ Loading post... }}
      }
    }

    BottomNav
  }
}

page profile "/profile" {
  div [class: "app-container"] {
    header [class: "profile-header"] {
      h2 {{ ${ctx.currentUser.username} }}
      button [onClick: showMenu, class: "btn-icon"] {
        i [class: "icon-menu"]
      }
    }

    main [class: "profile-content"] {
      section [class: "profile-info"] {
        img [src: ctx.currentUser.avatar, class: "profile-avatar"]

        div [class: "profile-stats"] {
          div [class: "stat"] {
            strong {{ ${ctx.currentUser.postsCount} }}
            span {{ posts }}
          }
          div [class: "stat"] {
            strong {{ ${ctx.currentUser.followersCount} }}
            span {{ followers }}
          }
          div [class: "stat"] {
            strong {{ ${ctx.currentUser.followingCount} }}
            span {{ following }}
          }
        }

        div [class: "profile-bio"] {
          h3 {{ ${ctx.currentUser.name} }}
          p {{ ${ctx.currentUser.bio} }}
        }

        button [class: "btn-edit-profile", onClick: editProfile] {
          {{ Edit Profile }}
        }
      }

      section [class: "profile-posts"] {
        div [class: "posts-grid"] @for(ctx.userPosts as post) {
          a [href: "#/posts/${post.id}", class: "grid-item"] {
            img [src: post.thumbnail]
            div [class: "grid-overlay"] {
              span {
                i [class: "icon-heart"]
                {{ ${post.likesCount} }}
              }
              span {
                i [class: "icon-comment"]
                {{ ${post.commentsCount} }}
              }
            }
          }
        }

        div @if(ctx.userPosts.length == 0) [class: "empty-posts"] {
          i [class: "icon-camera"]
          p {{ No posts yet }}
        }
      }
    }

    BottomNav
  }
}

page search "/search" {
  div [class: "app-container"] {
    header [class: "search-header"] {
      input [
        type: "search",
        bind: ctx.searchQuery,
        placeholder: "Search...",
        onInput: handleSearch,
        class: "search-input"
      ]
    }

    main [class: "search-results"] {
      div @if(ctx.searchResults.length > 0) {
        div @for(ctx.searchResults as post) {
          a [href: "#/posts/${post.id}", class: "search-result-item"] {
            img [src: post.thumbnail]
          }
        }
      } @else if ctx.searchQuery.length > 0 {
        div [class: "empty-results"] {
          p {{ No results found }}
        }
      } @else {
        div [class: "search-placeholder"] {
          i [class: "icon-search-big"]
          p {{ Search for posts }}
        }
      }
    }

    BottomNav
  }
}
```

### actions.ts

```typescript
export interface User {
  id: number;
  username: string;
  name: string;
  avatar: string;
  bio: string;
  postsCount: number;
  followersCount: number;
  followingCount: number;
}

export interface Post {
  id: number;
  user: User;
  image: string;
  thumbnail: string;
  caption: string;
  location?: string;
  likesCount: number;
  commentsCount: number;
  isLiked: boolean;
  isBookmarked: boolean;
  timeAgo: string;
  createdAt: number;
}

export interface Comment {
  id: number;
  user: User;
  text: string;
  likesCount: number;
  isLiked: boolean;
  timeAgo: string;
  createdAt: number;
}

export interface Story {
  id: number;
  user: User;
  seen: boolean;
}

export const actions = {
  // Navigate to post detail
  navigateToPost: (postId: number) => (ctx: any, event: Event) => {
    window.location.hash = `#/posts/${postId}`;
  },

  // Go back
  goBack: (ctx: any, event: Event) => {
    window.history.back();
  },

  // Toggle like
  toggleLike: (postId: number) => (ctx: any, event: Event) => {
    event.preventDefault();

    const post = ctx.data.posts.find((p: Post) => p.id === postId);
    if (post) {
      post.isLiked = !post.isLiked;
      post.likesCount += post.isLiked ? 1 : -1;
    }

    if (ctx.data.currentPost && ctx.data.currentPost.id === postId) {
      ctx.data.currentPost.isLiked = post.isLiked;
      ctx.data.currentPost.likesCount = post.likesCount;
    }

    ctx.rerender();
  },

  // Toggle bookmark
  toggleBookmark: (postId: number) => (ctx: any, event: Event) => {
    const post = ctx.data.posts.find((p: Post) => p.id === postId);
    if (post) {
      post.isBookmarked = !post.isBookmarked;
    }

    if (ctx.data.currentPost && ctx.data.currentPost.id === postId) {
      ctx.data.currentPost.isBookmarked = post.isBookmarked;
    }

    ctx.rerender();
  },

  // Submit comment
  submitComment: async (ctx: any, event: Event) => {
    const text = ctx.data.newComment.trim();
    if (!text || !ctx.data.currentPost) return;

    const newComment: Comment = {
      id: Date.now(),
      user: ctx.data.currentUser,
      text,
      likesCount: 0,
      isLiked: false,
      timeAgo: 'Just now',
      createdAt: Date.now()
    };

    ctx.data.comments.unshift(newComment);
    ctx.data.currentPost.commentsCount++;
    ctx.data.newComment = '';

    // Update post in main feed
    const post = ctx.data.posts.find((p: Post) => p.id === ctx.data.currentPost.id);
    if (post) {
      post.commentsCount++;
    }

    ctx.rerender();
  },

  // Toggle comment like
  toggleCommentLike: (commentId: number) => (ctx: any, event: Event) => {
    const comment = ctx.data.comments.find((c: Comment) => c.id === commentId);
    if (comment) {
      comment.isLiked = !comment.isLiked;
      comment.likesCount += comment.isLiked ? 1 : -1;
      ctx.rerender();
    }
  },

  // View story
  viewStory: (storyId: number) => (ctx: any, event: Event) => {
    const story = ctx.data.stories.find((s: Story) => s.id === storyId);
    if (story) {
      story.seen = true;
      ctx.rerender();
    }
    // In real app, open story viewer modal
    alert('Story viewer would open here');
  },

  // Search
  handleSearch: (ctx: any, event: Event) => {
    const query = ctx.data.searchQuery.toLowerCase();
    if (query.length === 0) {
      ctx.data.searchResults = [];
    } else {
      ctx.data.searchResults = ctx.data.posts.filter((post: Post) =>
        post.caption.toLowerCase().includes(query) ||
        post.user.username.toLowerCase().includes(query)
      );
    }
    ctx.rerender();
  },

  // Placeholder actions
  showPostOptions: (postId: number) => (ctx: any, event: Event) => {
    alert('Post options menu would open here');
  },

  sharePost: (postId: number) => (ctx: any, event: Event) => {
    alert('Share dialog would open here');
  },

  replyToComment: (commentId: number) => (ctx: any, event: Event) => {
    alert(`Reply to comment ${commentId}`);
  },

  openMessages: (ctx: any, event: Event) => {
    alert('Messages would open here');
  },

  showMenu: (ctx: any, event: Event) => {
    alert('Menu would open here');
  },

  editProfile: (ctx: any, event: Event) => {
    alert('Edit profile would open here');
  }
};
```

### main.ts

```typescript
import { setContext, router, getContext } from './dist/router';
import { initEvents } from './dist/events';
import { Post, User, Comment, Story } from './actions';

// Mock data
const currentUser: User = {
  id: 1,
  username: 'johndoe',
  name: 'John Doe',
  avatar: 'https://i.pravatar.cc/150?img=1',
  bio: 'Photography enthusiast 📸',
  postsCount: 42,
  followersCount: 1234,
  followingCount: 567
};

const mockPosts: Post[] = [
  {
    id: 1,
    user: {
      id: 2,
      username: 'jane_smith',
      name: 'Jane Smith',
      avatar: 'https://i.pravatar.cc/150?img=5',
      bio: '',
      postsCount: 0,
      followersCount: 0,
      followingCount: 0
    },
    image: 'https://picsum.photos/600/600?random=1',
    thumbnail: 'https://picsum.photos/300/300?random=1',
    caption: 'Beautiful sunset at the beach! 🌅',
    location: 'Malibu Beach',
    likesCount: 234,
    commentsCount: 12,
    isLiked: false,
    isBookmarked: false,
    timeAgo: '2 hours ago',
    createdAt: Date.now() - 7200000
  },
  {
    id: 2,
    user: {
      id: 3,
      username: 'travel_enthusiast',
      name: 'Alex Travel',
      avatar: 'https://i.pravatar.cc/150?img=8',
      bio: '',
      postsCount: 0,
      followersCount: 0,
      followingCount: 0
    },
    image: 'https://picsum.photos/600/600?random=2',
    thumbnail: 'https://picsum.photos/300/300?random=2',
    caption: 'Exploring the mountains ⛰️',
    location: 'Swiss Alps',
    likesCount: 567,
    commentsCount: 34,
    isLiked: true,
    isBookmarked: true,
    timeAgo: '5 hours ago',
    createdAt: Date.now() - 18000000
  }
];

const mockStories: Story[] = [
  { id: 1, user: { ...currentUser, id: 1 }, seen: false },
  { id: 2, user: { id: 2, username: 'jane_smith', name: 'Jane', avatar: 'https://i.pravatar.cc/150?img=5' } as User, seen: true },
  { id: 3, user: { id: 3, username: 'alex', name: 'Alex', avatar: 'https://i.pravatar.cc/150?img=8' } as User, seen: false }
];

// Initialize context
const initialContext = {
  currentUser,
  posts: mockPosts,
  stories: mockStories,
  userPosts: mockPosts.slice(0, 6),
  currentPost: null as Post | null,
  comments: [] as Comment[],
  newComment: '',
  searchQuery: '',
  searchResults: [] as Post[],
  isLoadingPosts: false,
  isLoadingComments: false,
  currentRoute: '/'
};

setContext(initialContext);

// Update current route on hash change
window.addEventListener('hashchange', () => {
  const ctx = getContext();
  const hash = window.location.hash;

  ctx.currentRoute = hash === '' ? '/' : hash.split('/')[0].slice(1) || '/';

  // Load post detail if navigating to post page
  if (hash.startsWith('#/posts/')) {
    const postId = parseInt(hash.split('/')[2]);
    const post = ctx.posts.find((p: Post) => p.id === postId);
    if (post) {
      ctx.currentPost = { ...post };
      // Mock comments
      ctx.comments = [
        {
          id: 1,
          user: currentUser,
          text: 'Great photo!',
          likesCount: 5,
          isLiked: false,
          timeAgo: '1 hour ago',
          createdAt: Date.now()
        }
      ];
    }
  } else {
    ctx.currentPost = null;
    ctx.comments = [];
  }
});

// Initialize
initEvents();
router.init();
```

## Key Features Demonstrated

- ✅ Multi-page navigation (feed, post detail, profile, search)
- ✅ Component composition with parameters
- ✅ Element directives (`@for`, `@if`)
- ✅ Conditional rendering and dynamic classes
- ✅ Event handling with parameters
- ✅ Two-way data binding for search and comments
- ✅ Route parameters (`/posts/:id`)
- ✅ Optimistic UI updates (instant like/bookmark)
- ✅ State management across pages
- ✅ Responsive grid layout

## Next Steps

To make this production-ready, add:

- Real API integration
- Image upload functionality
- Infinite scroll for feed
- Story viewer with swipe gestures
- Push notifications
- Real-time updates (WebSocket)
- Image optimization and lazy loading
- Authentication and protected routes
